use crate::entities::chat::Model as Chat;

use axum::extract::FromRef;
use sea_orm::DatabaseConnection;
use tokio::sync::broadcast;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub conn: DatabaseConnection,
    pub queue: broadcast::Sender<Chat>,
}
