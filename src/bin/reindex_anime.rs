use animethemes_graphql_rust::entities::content::anime;
use animethemes_graphql_rust::typesense::documents::anime_document::{
    AnimeDocument, build_anime_documents,
};
use animethemes_graphql_rust::typesense::index_document::index_document;
use anyhow::{Context, Result};

use animethemes_graphql_rust::db::connect;
use animethemes_graphql_rust::typesense::client::create_typesense_client;

use typesense::prelude::Document;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let database = connect().await;
    let typesense = create_typesense_client();

    let collection = typesense.collection::<AnimeDocument>();

    match collection.retrieve().await {
        Ok(_) => {
            collection
                .delete()
                .await
                .context("failed to delete existing Typesense collection")?;
        }
        Err(_) => {}
    }

    typesense
        .collections()
        .create(AnimeDocument::collection_schema())
        .await
        .context("failed to create Typesense collection")?;

    index_document::<anime::Entity, AnimeDocument, _>(
        &database,
        &typesense,
        anime::Entity,
        build_anime_documents,
    )
    .await
    .unwrap();

    Ok(())
}
