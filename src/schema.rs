use async_graphql::{EmptyMutation, EmptySubscription, Schema, http::GraphiQLSource};

use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{extract::State, response::Html};
use sea_orm::DatabaseConnection;

use crate::graphql::{loaders::loaders::RegisterLoaders, query::Query};

pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub fn create_schema(db: DatabaseConnection) -> AppSchema {
    Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(db.clone())
        .register_loaders(db)
        .finish()
}

pub async fn graphql_handler(
    State(schema): State<AppSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

pub async fn graphiql() -> Html<String> {
    Html(GraphiQLSource::build().endpoint("/").finish())
}
