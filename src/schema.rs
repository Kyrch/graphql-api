use async_graphql::{EmptySubscription, Schema, http::GraphiQLSource};

use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{extract::State, response::Html};
use sea_orm::DatabaseConnection;

use crate::{
    graphql::{loaders::loaders::RegisterLoaders, mutation::Mutation, query::Query},
    typesense::client::create_typesense_client,
};

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn create_schema(db: DatabaseConnection) -> AppSchema {
    let typesense = create_typesense_client();

    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(db.clone())
        .data(typesense)
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
