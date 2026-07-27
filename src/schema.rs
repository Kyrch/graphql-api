use async_graphql::{
    EmptyMutation, EmptySubscription, Schema, dataloader::DataLoader, http::GraphiQLSource,
};

use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{extract::State, response::Html};
use sea_orm::DatabaseConnection;

use crate::{graphql::query::Query, loaders::anime_synonyms::AnimeSynonymsLoader};

pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub fn create_schema(db: DatabaseConnection) -> AppSchema {
    let anime_synonyms_loader =
        DataLoader::new(AnimeSynonymsLoader { db: db.clone() }, tokio::spawn);

    Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(db)
        .data(anime_synonyms_loader)
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
