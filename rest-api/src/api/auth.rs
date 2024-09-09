use crate::entities::{prelude::Users, users::Column};
use crate::utils::app_error::AppError;
use crate::utils::hash::verify_password;
use crate::utils::jwt::create_token;
use axum::http::StatusCode;
use axum::{extract::State, Json};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RequestUser {
    username: String,
    password: String,
}

pub async fn login(
    State(db): State<DatabaseConnection>,
    Json(request_user): Json<RequestUser>,
) -> Result<String, AppError> {
    let user = Users::find()
        .filter(Column::Username.eq(request_user.username))
        .one(&db)
        .await
        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Database error"))?
        .ok_or_else(|| AppError::new(StatusCode::NOT_FOUND, "User not found"))?;

    if !verify_password(&request_user.password, &user.password)? {
        return Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "incorrect username and/or password",
        ));
    }

    Ok(create_token(user.username.clone())?)
}
