mod db;
mod entities;
mod graphql;

use async_graphql::{
    http::{GraphiQLSource},
    EmptyMutation,
    EmptySubscription,
    Schema,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    response::Html,
    routing::get,
    Router,
};

use graphql::query::Query;

type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;

async fn graphql_handler(
    State(schema): State<AppSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphiql() -> Html<String> {
    Html(GraphiQLSource::build().endpoint("/").finish())
}

#[tokio::main]
async fn main() {
    let db = db::connect().await;

    let schema = Schema::build(
        Query,
        EmptyMutation,
        EmptySubscription,
    )
    .data(db)
    .finish();

    let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .with_state(schema);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    println!("GraphiQL: http://localhost:8000");

    axum::serve(listener, app)
        .await
        .unwrap();
}