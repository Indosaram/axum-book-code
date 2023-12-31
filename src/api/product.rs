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
    entities::{
        prelude::Product,
        product::{ActiveModel, Column, Model},
    },
    utils::app_error::AppError,
};

pub async fn get_product(
    State(conn): State<DatabaseConnection>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<Model>>, AppError> {
    let mut condition = Condition::all();

    if let Some(id) = params.get("id") {
        match id.parse::<i32>() {
            Ok(parsed_id) => condition = condition.add(Column::Id.eq(parsed_id)),
            Err(_) => {
                return Err(AppError::new(
                    StatusCode::BAD_REQUEST,
                    "ID must be an integer",
                ))
            }
        }
    }

    if let Some(title) = params.get("title") {
        condition = condition.add(Column::Title.contains(title));
    }
    if let Some(price) = params.get("price") {
        condition = condition.add(Column::Price.contains(price));
    }
    if let Some(category) = params.get("category") {
        condition = condition.add(Column::Category.contains(category));
    }

    match Product::find().filter(condition).all(&conn).await {
        Ok(products) => Ok(Json(products)),
        Err(_) => Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database error",
        )),
    }
}

#[derive(serde::Deserialize)]
pub struct UpsertModel {
    id: Option<i32>,
    title: Option<String>,
    price: Option<i32>,
    category: Option<String>,
}

pub async fn post_product(
    State(conn): State<DatabaseConnection>,
    Json(product): Json<UpsertModel>,
) -> Result<Json<Model>, AppError> {
    let new_product = ActiveModel {
        id: ActiveValue::NotSet,
        title: ActiveValue::Set(product.title.unwrap()),
        price: ActiveValue::Set(product.price.unwrap()),
        category: ActiveValue::Set(product.category.unwrap()),
    };

    match new_product.insert(&conn).await {
        Ok(inserted_product) => Ok(Json(inserted_product)),
        Err(_) => Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database error",
        )),
    }
}

pub async fn put_product(
    State(conn): State<DatabaseConnection>,
    Json(product): Json<UpsertModel>,
) -> Result<Json<Model>, AppError> {
    let result = match Product::find_by_id(product.id.unwrap()).one(&conn).await {
        Ok(result) => result.ok_or(AppError::new(StatusCode::NOT_FOUND, "Product not found"))?,
        Err(_) => {
            return Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error",
            ))
        }
    };

    let new_product = ActiveModel {
        id: ActiveValue::Set(result.id),
        title: ActiveValue::Set(product.title.unwrap_or(result.title)),
        price: ActiveValue::Set(product.price.unwrap_or(result.price)),
        category: ActiveValue::Set(product.category.unwrap_or(result.category)),
    };

    match new_product.update(&conn).await {
        Ok(updated_product) => Ok(Json(updated_product)),
        Err(_) => Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database error",
        )),
    }
}

pub async fn delete_product(
    State(conn): State<DatabaseConnection>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<&'static str>, AppError> {
    let mut condition = Condition::any();

    if let Some(id) = params.get("id") {
        condition = condition.add(Column::Id.eq(id.parse::<i32>().unwrap()));
    }

    if let Some(title) = params.get("title") {
        condition = condition.add(Column::Title.contains(title));
    }
    if let Some(price) = params.get("price") {
        condition = condition.add(Column::Price.contains(price));
    }
    if let Some(category) = params.get("category") {
        condition = condition.add(Column::Category.contains(category));
    }

    let product = match Product::find().filter(condition).one(&conn).await {
        Ok(product) => product.ok_or(AppError::new(StatusCode::NOT_FOUND, "Product not found"))?,
        Err(_) => {
            return Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error",
            ))
        }
    };

    match product.delete(&conn).await {
        Ok(_) => Ok(Json("Deleted")),
        Err(_) => Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database error",
        )),
    }
}
