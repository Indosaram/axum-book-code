mod api;
mod db;
mod entities;

use axum::{routing::get, Router};

use api::category::{delete_category, get_category, post_category};
use api::product::{delete_product, get_product, post_product, put_product};
use api::users::{delete_user, get_user, post_user, put_user};
use db::init_db;

#[tokio::main]
async fn main() {
    println!("Connecting to DB...");
    dotenvy::dotenv().ok();
    let conn = init_db().await;

    println!("Starting server...");
    let app = Router::new()
        .route(
            "/users",
            get(get_user)
                .post(post_user)
                .put(put_user)
                .delete(delete_user),
        )
        .route(
            "/category",
            get(get_category)
                .post(post_category)
                .delete(delete_category),
        )
        .route(
            "/product",
            get(get_product)
                .post(post_product)
                .put(put_product)
                .delete(delete_product),
        )
        .with_state(conn);

    axum::Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
