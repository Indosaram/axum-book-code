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
    prelude::Product,
    product::{ActiveModel, Column, Model},
};

pub async fn get_product(
    State(conn): State<DatabaseConnection>,
    Query(params): Query<HashMap<String, String>>,
) -> Json<Vec<Model>> {
    let mut condition = Condition::all();

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

    Json(Product::find().filter(condition).all(&conn).await.unwrap())
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
) -> Json<Model> {
    let new_product = ActiveModel {
        id: ActiveValue::NotSet,
        title: ActiveValue::Set(product.title.unwrap()),
        price: ActiveValue::Set(product.price.unwrap()),
        category: ActiveValue::Set(product.category.unwrap()),
    };

    Json(new_product.insert(&conn).await.unwrap())
}

pub async fn put_product(
    State(conn): State<DatabaseConnection>,
    Json(product): Json<UpsertModel>,
) -> Json<Model> {
    let result = Product::find_by_id(product.id.unwrap())
        .one(&conn)
        .await
        .unwrap()
        .unwrap();

    let new_product = ActiveModel {
        id: ActiveValue::Set(result.id),
        title: ActiveValue::Set(product.title.unwrap_or(result.title)),
        price: ActiveValue::Set(product.price.unwrap_or(result.price)),
        category: ActiveValue::Set(product.category.unwrap_or(result.category)),
    };

    Json(new_product.update(&conn).await.unwrap())
}

pub async fn delete_product(
    State(conn): State<DatabaseConnection>,
    Query(params): Query<HashMap<String, String>>,
) -> Json<&'static str> {
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

    let product = Product::find()
        .filter(condition)
        .one(&conn)
        .await
        .unwrap()
        .unwrap();

    product.delete(&conn).await.unwrap();

    Json("Deleted")
}
