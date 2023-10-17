mod entities;

use std::collections::HashMap;

use axum::{
    extract::Query,
    routing::{delete, get, post, put},
    Json, Router,
};

use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, Database, EntityTrait, ModelTrait,
    QueryFilter,
};

use entities::{
    prelude::Users,
    users::{ActiveModel, Column, Model},
};

const DATABASE_URL: &str = "postgres://axum:1234@localhost/axum";

async fn get_user(Query(params): Query<HashMap<String, String>>) -> Json<Model> {
    let db = Database::connect(DATABASE_URL).await.unwrap();

    let mut condition = Condition::any();

    if let Some(id) = params.get("id") {
        condition = condition.add(Column::Id.eq(id.parse::<i32>().unwrap()));
    }

    if let Some(username) = params.get("username") {
        condition = condition.add(Column::Username.contains(username));
    }

    let user = Users::find()
        .filter(condition)
        .one(&db)
        .await
        .unwrap()
        .unwrap();

    Json(user)
}

async fn post_user(Json(user): Json<Model>) -> Json<Model> {
    let db = Database::connect(DATABASE_URL).await.unwrap();

    let new_user = ActiveModel {
        id: ActiveValue::NotSet,
        username: ActiveValue::Set(user.username),
        password: ActiveValue::Set(user.password),
    };

    let result = new_user.insert(&db).await.unwrap();

    Json(result)
}

#[derive(serde::Deserialize)]
struct UpdateModel {
    id: i32,
    username: String,
    password: Option<String>,
}

async fn put_user(Json(user): Json<UpdateModel>) -> Json<&'static str> {
    let db = Database::connect(DATABASE_URL).await.unwrap();

    let result = Users::find_by_id(user.id).one(&db).await.unwrap().unwrap();

    let new_user = ActiveModel {
        id: ActiveValue::Set(user.id),
        username: ActiveValue::Set(user.username),
        password: ActiveValue::Set(user.password.unwrap_or(result.password)),
    };

    new_user.update(&db).await.unwrap();

    Json("Updated")
}

async fn delete_user(Query(params): Query<HashMap<String, String>>) -> Json<&'static str> {
    let db = Database::connect(DATABASE_URL).await.unwrap();

    let mut condition = Condition::any();

    if let Some(id) = params.get("id") {
        condition = condition.add(Column::Id.eq(id.parse::<i32>().unwrap()));
    }

    if let Some(username) = params.get("username") {
        condition = condition.add(Column::Username.contains(username));
    }

    let user = Users::find()
        .filter(condition)
        .one(&db)
        .await
        .unwrap()
        .unwrap();

    user.delete(&db).await.unwrap();

    Json("Deleted")
}

#[tokio::main]
async fn main() {
    println!("Starting server...");
    let app = Router::new()
        .route("/users", get(get_user))
        .route("/users", post(post_user))
        .route("/users", put(put_user))
        .route("/users", delete(delete_user));

    axum::Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}