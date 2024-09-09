use std::collections::HashMap;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};

use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DatabaseConnection, EntityTrait, Order,
    QueryFilter, QueryOrder,
};

use crate::{
    entities::users::{ActiveModel, Column, Entity, Model},
    utils::{app_error::AppError, hash::hash_password},
};

pub async fn get_users(
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

    if let Some(username) = params.get("username") {
        condition = condition.add(Column::Username.contains(username));
    }

    match Entity::find()
        .filter(condition)
        .order_by(Column::Username, Order::Asc)
        .all(&conn)
        .await
    {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database error",
        )),
    }
}

#[derive(serde::Deserialize)]
pub struct UpsertModel {
    id: Option<i32>,
    username: Option<String>,
    password: Option<String>,
}

pub async fn post_user(
    State(conn): State<DatabaseConnection>,
    Json(user): Json<UpsertModel>,
) -> Result<Json<Model>, AppError> {
    if user.username.is_none() || user.password.is_none() {
        return Err(AppError::new(
            StatusCode::BAD_REQUEST,
            "Username or password not provided",
        ));
    }

    let hashed_password = hash_password(&user.password.unwrap())?;

    let new_user = ActiveModel {
        id: ActiveValue::NotSet,
        username: ActiveValue::Set(user.username.unwrap()),
        password: ActiveValue::Set(hashed_password),
    };

    match new_user.insert(&conn).await {
        Ok(result) => Ok(Json(result)),
        Err(_) => Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database error",
        )),
    }
}

pub async fn put_user(
    State(conn): State<DatabaseConnection>,
    Json(user): Json<UpsertModel>,
) -> Result<Json<Model>, AppError> {
    let id = match user.id {
        Some(id) => id,
        None => {
            return Err(AppError::new(
                StatusCode::BAD_REQUEST,
                "User ID not provided",
            ))
        }
    };

    let found_user = match Entity::find_by_id(id).one(&conn).await {
        Ok(user) => user.ok_or(AppError::new(StatusCode::NOT_FOUND, "User not found"))?,
        Err(_) => {
            return Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error",
            ))
        }
    };

    let mut active_user: ActiveModel = found_user.into();
    active_user.username = user
        .username
        .map(ActiveValue::Set)
        .unwrap_or(active_user.username);
    active_user.password = match user.password {
        Some(password) => ActiveValue::Set(hash_password(&password)?),
        None => active_user.password,
    };

    match active_user.update(&conn).await {
        Ok(result) => Ok(Json(result)),
        Err(_) => Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database error",
        )),
    }
}

pub async fn delete_user(
    State(conn): State<DatabaseConnection>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<&'static str>, AppError> {
    let id = match params.get("id") {
        Some(id) => id,
        None => {
            return Err(AppError::new(
                StatusCode::BAD_REQUEST,
                "User ID not provided",
            ))
        }
    };

    match Entity::delete_by_id(
        id.parse::<i32>()
            .map_err(|_| AppError::new(StatusCode::BAD_REQUEST, "ID must be an integer"))?,
    )
    .exec(&conn)
    .await
    {
        Ok(_) => Ok(Json("User deleted")),
        Err(_) => Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database error",
        )),
    }
}
