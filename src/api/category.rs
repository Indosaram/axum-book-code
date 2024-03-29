use std::collections::HashMap;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};

use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    ModelTrait, QueryFilter,
};

use crate::{
    entities::category::{ActiveModel, Column, Entity, Model},
    utils::app_error::AppError,
};

pub async fn get_category(
    State(conn): State<DatabaseConnection>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<Model>>, AppError> {
    let mut condition = Condition::all();

    if let Some(name) = params.get("name") {
        condition = condition.add(Column::Name.contains(name));
    }

    match Entity::find().filter(condition).all(&conn).await {
        Ok(categories) => Ok(Json(categories)),
        Err(_) => Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database error",
        )),
    }
}

pub async fn post_category(
    State(conn): State<DatabaseConnection>,
    Json(category): Json<Model>,
) -> Result<Json<Model>, AppError> {
    let new_category = ActiveModel {
        name: ActiveValue::Set(category.name),
    };

    match new_category.insert(&conn).await {
        Ok(result) => Ok(Json(result)),
        Err(_) => Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database error",
        )),
    }
}

pub async fn delete_category(
    State(conn): State<DatabaseConnection>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<&'static str>, AppError> {
    if params.get("name").is_none() {
        return Err(AppError::new(StatusCode::BAD_REQUEST, "Name is required"));
    }

    let category = match Entity::find()
        .filter(Condition::any().add(Column::Name.contains(params.get("name").unwrap())))
        .one(&conn)
        .await
    {
        Ok(Some(category)) => category,
        Ok(None) => return Err(AppError::new(StatusCode::NOT_FOUND, "Category not found")),
        Err(_) => {
            return Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error",
            ))
        }
    };

    match category.delete(&conn).await {
        Ok(_) => Ok(Json("Deleted")),
        Err(_) => Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database error",
        )),
    }
}
