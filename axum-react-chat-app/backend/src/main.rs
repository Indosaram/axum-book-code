mod api;
mod entities;

use api::{
    chat::{get_chat, send, subscribe},
    chat_room::{delete_room, get_room, post_room, put_room},
    state::AppState,
    user::{delete_user, get_user, post_user, put_user},
};

use axum::{
    routing::{get, post},
    Router,
};

use migration::{Migrator, MigratorTrait};
use sea_orm::SqlxPostgresConnector;
use sqlx::PgPool;
use tokio::sync::broadcast;
use tower_http::{
    cors::{Any, CorsLayer},
    services::{ServeDir, ServeFile},
};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let state = AppState {
        conn: SqlxPostgresConnector::from_sqlx_postgres_pool(pool),
        queue: broadcast::channel(10).0,
    };

    Migrator::up(&state.conn, None).await.unwrap();

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

    Ok(app.into())
}
