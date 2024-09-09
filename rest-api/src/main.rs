mod api;
mod db;
mod entities;
mod utils;

use std::time::Duration;

use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use tower_http::{compression::CompressionLayer, timeout::TimeoutLayer, trace::TraceLayer};
use tracing::info;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use api::auth::login;
use api::category::{delete_category, get_category, post_category};
use api::product::{delete_product, get_product, post_product, put_product};
use api::users::{delete_user, get_users, post_user, put_user};

use db::init_db;

use utils::jwt::authenticate;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    info!("Connecting to DB...");
    let conn = init_db().await;

    info!("Starting server...");
    let app = Router::new()
        .route("/users", get(get_users).put(put_user).delete(delete_user))
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
        .route_layer(middleware::from_fn(authenticate))
        .route("/auth/login", post(login))
        .route("/auth/signup", post(post_user))
        .with_state(conn)
        .layer(TimeoutLayer::new(Duration::from_millis(1000)))
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
