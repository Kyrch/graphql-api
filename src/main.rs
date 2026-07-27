mod db;
mod entities;
mod graphql;
mod loaders;
mod schema;

use axum::{Router, routing::get};

use crate::schema::{graphiql, graphql_handler};

#[tokio::main]
async fn main() {
    let db = db::connect().await;

    let schema = schema::create_schema(db);

    let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .with_state(schema);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    println!("GraphiQL: http://localhost:8000");

    axum::serve(listener, app).await.unwrap();
}
