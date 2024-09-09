mod db;

use std::sync::{Arc, Mutex};

use db::init::init_db;

use axum::{debug_handler, extract::State, routing::get, Router};
use sea_orm::{ConnectionTrait, DatabaseBackend, DatabaseConnection, Statement};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let db = init_db().await;

    let app = Router::new().route("/", get(handler)).with_state(db);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[debug_handler]
async fn handler(State(db): State<DatabaseConnection>) -> String {
    let result = db
        .query_one(Statement::from_string(
            DatabaseBackend::Postgres,
            "SELECT * FROM users",
        ))
        .await
        .unwrap()
        .unwrap();

    let username: String = result.try_get("", "username").unwrap();

    username
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;
    use sea_orm::{
        entity::prelude::*, entity::*, tests_cfg::*, DatabaseBackend, MockDatabase, Transaction,
    };

    #[tokio::test]
    async fn test_handler() {
        dotenvy::dotenv().ok();
        // let db = init_db().await;

        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([vec![vec![User {
                id: 1,
                username: "axum".to_owned(),
            }]]])
            .into_connection();
        let app = Router::new().route("/", get(handler)).with_state(db);

        let server = TestServer::new(app).unwrap();

        let response = server.get("/").await;

        response.assert_status_ok();
        response.assert_text("axum");
    }
}
