use sea_orm::{DatabaseConnection, LoaderTrait};
use serde::{Deserialize, Serialize};
use typesense::Typesense;

use crate::{
    entities::content::{anime, series},
    typesense::{
        documents::{
            HasId,
            anime_document::{AnimeDocument, build_anime_documents},
        },
        index_document::BuildDocumentsFuture,
    },
};

pub const QUERY_BY: &str = "title,anime.title_english,anime.title_native,anime.synonyms";
pub const QUERY_BY_WEIGHTS: &str = "10,8,8,6";

#[derive(Debug, Clone, Serialize, Deserialize, Typesense)]
#[typesense(collection_name = "series", enable_nested_fields = true)]
pub struct SeriesDocument {
    pub id: String,
    pub title: String,
    #[typesense(flatten)]
    pub anime: AnimeDocument,
    pub created_at: Option<i64>,
}

impl HasId for SeriesDocument {
    fn id(&self) -> &str {
        &self.id
    }
}

type SeriesDocumentFrom = (series::Model, AnimeDocument);

impl From<SeriesDocumentFrom> for SeriesDocument {
    fn from((model, anime_document): SeriesDocumentFrom) -> Self {
        Self {
            id: model.id.to_string(),
            title: model.title,
            anime: anime_document,
            created_at: model.created_at.map(|c| c.timestamp()),
        }
    }
}

pub fn build_series_documents<'a>(
    models: Vec<series::Model>,
    database: &'a DatabaseConnection,
) -> BuildDocumentsFuture<'a, SeriesDocument> {
    Box::pin(async move {
        let anime_models: Vec<anime::Model> = models
            .load_many(anime::Entity, database)
            .await?
            .into_iter()
            .flatten()
            .collect();

        let anime_documents = build_anime_documents(anime_models, database).await?;

        let documents = models
            .into_iter()
            .zip(anime_documents)
            .map(|(model, anime_document)| SeriesDocument::from((model, anime_document)))
            .collect();

        Ok(documents)
    })
}
