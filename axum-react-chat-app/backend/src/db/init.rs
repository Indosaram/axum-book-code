use std::env;

use sea_orm::{Database, DatabaseConnection};

pub async fn init_db() -> DatabaseConnection {
    match Database::connect(&env::var("DATABASE_URL").expect("DATABASE_URL not set")).await {
        Ok(db) => db,
        Err(e) => panic!("Error connecting to database: {}", e),
    }
}
