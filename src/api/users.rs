use std::collections::HashMap;

use axum::{
    extract::{Query, State},
    Json,
};

use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    ModelTrait, QueryFilter,
};

use crate::{
    entities::{
        prelude::Users,
        users::{ActiveModel, Column, Model},
    },
    utils::{app_error::AppError, hash::hash_password},
};

pub async fn get_user(
    State(conn): State<DatabaseConnection>,
    Query(params): Query<HashMap<String, String>>,
) -> Json<Vec<Model>> {
    let mut condition = Condition::all();

    if let Some(id) = params.get("id") {
        condition = condition.add(Column::Id.eq(id.parse::<i32>().unwrap()));
    }

    if let Some(username) = params.get("username") {
        condition = condition.add(Column::Username.contains(username));
    }

    Json(Users::find().filter(condition).all(&conn).await.unwrap())
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
    let hashed_password = hash_password(&user.password.unwrap())?;

    let new_user = ActiveModel {
        id: ActiveValue::NotSet,
        username: ActiveValue::Set(user.username.unwrap()),
        password: ActiveValue::Set(hashed_password),
    };

    let result = new_user.insert(&conn).await.unwrap();

    Ok(Json(result))
}

pub async fn put_user(
    State(conn): State<DatabaseConnection>,
    Json(user): Json<UpsertModel>,
) -> Json<Model> {
    let result = Users::find_by_id(user.id.unwrap())
        .one(&conn)
        .await
        .unwrap()
        .unwrap();

    let new_user = ActiveModel {
        id: ActiveValue::Set(result.id),
        username: ActiveValue::Set(user.username.unwrap_or(result.username)),
        password: ActiveValue::Set(user.password.unwrap_or(result.password)),
    };

    Json(new_user.update(&conn).await.unwrap())
}

pub async fn delete_user(
    State(conn): State<DatabaseConnection>,
    Query(params): Query<HashMap<String, String>>,
) -> Json<&'static str> {
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    let mut condition = Condition::any();

    if let Some(id) = params.get("id") {
        condition = condition.add(Column::Id.eq(id.parse::<i32>().unwrap()));
    }

    if let Some(username) = params.get("username") {
        condition = condition.add(Column::Username.contains(username));
    }

    let user = Users::find()
        .filter(condition)
        .one(&conn)
        .await
        .unwrap()
        .unwrap();

    user.delete(&conn).await.unwrap();

    Json("Deleted")
}
