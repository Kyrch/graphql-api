use sea_orm::{Database, DatabaseConnection};
use dotenvy::dotenv;
use std::env;

pub async fn connect() -> DatabaseConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL is not defined");

    Database::connect(database_url)
        .await
        .unwrap()
}