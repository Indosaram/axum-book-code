mod api;
mod db;
mod entities;

use api::{
    chat::{get_chat, send, subscribe},
    chat_room::{delete_room, get_room, post_room, put_room},
    state::AppState,
    user::{delete_user, get_user, post_user, put_user},
};
use db::init::init_db;

use axum::{
    routing::{get, post},
    Router,
};

use tokio::sync::broadcast;
use tower_http::{
    cors::{Any, CorsLayer},
    services::{ServeDir, ServeFile},
};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let state = AppState {
        conn: init_db().await,
        queue: broadcast::channel(10).0,
    };

    let app = Router::new()
        .nest(
            "/chat",
            Router::new()
                .route("/", get(get_chat))
                .route("/subscribe", get(subscribe))
                .route("/send", post(send)),
        )
        .route(
            "/room",
            get(get_room)
                .post(post_room)
                .put(put_room)
                .delete(delete_room),
        )
        .route(
            "/user",
            get(get_user)
                .post(post_user)
                .put(put_user)
                .delete(delete_user),
        )
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_headers(Any)
                .allow_methods(Any),
        )
        .nest_service(
            "/",
            ServeDir::new("static").not_found_service(ServeFile::new("static/index.html")),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
