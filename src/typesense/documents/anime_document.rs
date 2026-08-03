use sea_orm::{ActiveEnum, ColumnTrait, DatabaseConnection, EntityTrait, LoaderTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use typesense::Typesense;

use crate::{
    entities::content::{anime, synonym},
    typesense::index_document::BuildDocumentsFuture,
};

#[derive(Debug, Clone, Serialize, Deserialize, Typesense)]
#[typesense(collection_name = "anime")]
pub struct AnimeDocument {
    pub id: String,
    #[typesense(sort)]
    pub title: String,
    #[typesense(sort)]
    pub title_english: Option<String>,
    #[typesense(sort)]
    pub title_native: Option<String>,
    pub format: Option<i32>,
    pub season: Option<i32>,
    #[typesense(sort)]
    pub year: Option<i32>,
    pub created_at: Option<i64>,
    pub synonyms: Vec<String>,
}

impl From<(anime::Model, Vec<synonym::Model>)> for AnimeDocument {
    fn from((model, synonyms): (anime::Model, Vec<synonym::Model>)) -> Self {
        Self {
            id: model.id.to_string(),
            title: model.title.clone(),
            title_english: model.title_english.clone(),
            title_native: model.title_native.clone(),
            format: model.format.map(|f| f.to_value()),
            season: model.season.map(|s| s.to_value()),
            year: model.year,
            created_at: model.created_at.map(|c| c.timestamp()),
            synonyms: synonyms.iter().map(|s| s.text.clone()).collect(),
        }
    }
}

pub fn build_anime_documents<'a>(
    models: Vec<anime::Model>,
    database: &'a DatabaseConnection,
) -> BuildDocumentsFuture<'a, AnimeDocument> {
    Box::pin(async move {
        let synonyms: Vec<Vec<synonym::Model>> = models
            .load_many(
                synonym::Entity::find().filter(synonym::Column::SynonymableType.eq("anime")),
                database,
            )
            .await?;

        let documents = models
            .into_iter()
            .zip(synonyms)
            .map(|(model, synonyms)| AnimeDocument::from((model, synonyms)))
            .collect();

        Ok(documents)
    })
}
