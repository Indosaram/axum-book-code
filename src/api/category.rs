use std::collections::HashMap;

use axum::{
    extract::{Query, State},
    Json,
};

use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    ModelTrait, QueryFilter,
};

use crate::entities::{
    category::{ActiveModel, Column, Model},
    prelude::Category,
};

pub async fn get_category(
    State(conn): State<DatabaseConnection>,
    Query(params): Query<HashMap<String, String>>,
) -> Json<Vec<Model>> {
    let mut condition = Condition::all();

    if let Some(name) = params.get("name") {
        condition = condition.add(Column::Name.contains(name));
    }

    let user = Category::find().filter(condition).all(&conn).await.unwrap();

    Json(user)
}

pub async fn post_category(
    State(conn): State<DatabaseConnection>,
    Json(category): Json<Model>,
) -> Json<Model> {
    let new_category = ActiveModel {
        name: ActiveValue::Set(category.name),
    };

    Json(new_category.insert(&conn).await.unwrap())
}

pub async fn delete_category(
    State(conn): State<DatabaseConnection>,
    Query(params): Query<HashMap<String, String>>,
) -> Json<&'static str> {
    let mut condition = Condition::any();

    if let Some(name) = params.get("name") {
        condition = condition.add(Column::Name.contains(name));
    }

    let category = Category::find()
        .filter(condition)
        .one(&conn)
        .await
        .unwrap()
        .unwrap();

    category.delete(&conn).await.unwrap();

    Json("Deleted")
}
