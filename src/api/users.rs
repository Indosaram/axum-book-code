use std::collections::HashMap;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};

use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DatabaseConnection, EntityTrait, InsertResult, Order, QueryFilter, QueryOrder
};

use crate::{
    entities::users::{
        ActiveModel as UsersActiveModel, Column, Entity as UsersEntity, Model as UsersModel,
    },
    utils::{app_error::AppError, hash::hash_password},
};

pub async fn get_users(
    State(conn): State<DatabaseConnection>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<UsersModel>>, AppError> {
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

    match UsersEntity::find()
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
) -> Result<Json<UsersModel>, AppError> {
    // Check if password is provided
    let password = match &user.password {
        Some(password) => password,
        None => {
            return Err(AppError::new(
                StatusCode::BAD_REQUEST,
                "Password not provided",
            ))
        }
    };

    let hashed_password = hash_password(password)?;

    let new_user = UsersActiveModel {
        id: ActiveValue::NotSet,
        username: ActiveValue::Set(user.username.unwrap_or_default()),
        password: ActiveValue::Set(hashed_password),
    };

    let result: InsertResult<UsersActiveModel> =
        UsersEntity::insert(new_user).exec(&conn).await.unwrap();

    // Insert the new user into the database
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
) -> Result<Json<UsersModel>, AppError> {
    // Check if user id is provided
    let id = match user.id {
        Some(id) => id,
        None => {
            return Err(AppError::new(
                StatusCode::BAD_REQUEST,
                "User ID not provided",
            ))
        }
    };

    // Fetch the existing user from the database
    let found_user = match UsersEntity::find_by_id(id).one(&conn).await {
        Ok(user) => user.ok_or(AppError::new(StatusCode::NOT_FOUND, "User not found"))?,
        Err(_) => {
            return Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error",
            ))
        }
    };

    let mut active_user: UsersActiveModel = found_user.into();
    active_user.username = user
        .username
        .map(ActiveValue::Set)
        .unwrap_or(active_user.username);
    active_user.password = match user.password {
        Some(password) => ActiveValue::Set(hash_password(&password)?),
        None => active_user.password,
    };

    // Update the user in the database
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
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    let id = match params.get("id") {
        Some(id) => id,
        None => {
            return Err(AppError::new(
                StatusCode::BAD_REQUEST,
                "User ID not provided",
            ))
        }
    };

    match UsersEntity::delete_by_id(
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
