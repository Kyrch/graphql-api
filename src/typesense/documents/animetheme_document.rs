use sea_orm::{ActiveEnum, ColumnTrait, DatabaseConnection, EntityTrait, LoaderTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use typesense::Typesense;

use crate::{
    entities::content::{anime, animetheme::animetheme, song, synonym},
    typesense::{
        documents::{HasId, anime_document::AnimeDocument, song_document::SongDocument},
        index_document::BuildDocumentsFuture,
    },
};

pub const QUERY_BY: &str = "song.title,song.title_native,anime.title,anime.title_english,anime.title_native,anime.synonyms,type_sequence";
pub const QUERY_BY_WEIGHTS: &str = "10,8,6,5,5,4,4";

#[derive(Debug, Clone, Serialize, Deserialize, Typesense)]
#[typesense(collection_name = "animethemes", enable_nested_fields = true)]
pub struct AnimeThemeDocument {
    pub id: String,
    pub r#type: i32,
    pub sequence: Option<i32>,
    pub type_sequence: String,
    #[typesense(flatten)]
    pub anime: AnimeDocument,
    #[typesense(flatten)]
    pub song: Option<SongDocument>,
    pub created_at: Option<i64>,
}

impl HasId for AnimeThemeDocument {
    fn id(&self) -> &str {
        &self.id
    }
}

impl
    From<(
        animetheme::Model,
        anime::Model,
        Vec<synonym::Model>,
        Option<song::Model>,
    )> for AnimeThemeDocument
{
    fn from(
        (model, anime, synonyms, song): (
            animetheme::Model,
            anime::Model,
            Vec<synonym::Model>,
            Option<song::Model>,
        ),
    ) -> Self {
        Self {
            id: model.id.to_string(),
            r#type: model.r#type.to_value(),
            sequence: model.sequence,
            type_sequence: format!("{}{}", model.r#type.localize(), model.sequence.unwrap_or(1)),
            anime: (anime, synonyms).into(),
            song: song.map(SongDocument::from),
            created_at: model.created_at.map(|c| c.timestamp()),
        }
    }
}

pub fn build_animetheme_documents<'a>(
    models: Vec<animetheme::Model>,
    database: &'a DatabaseConnection,
) -> BuildDocumentsFuture<'a, AnimeThemeDocument> {
    Box::pin(async move {
        let anime_models: Vec<anime::Model> = models
            .load_one(anime::Entity, database)
            .await?
            .into_iter()
            .map(|anime| anime.expect("Anime not found for animetheme"))
            .collect();

        let song_models: Vec<Option<song::Model>> = models.load_one(song::Entity, database).await?;

        let synonyms: Vec<Vec<synonym::Model>> = anime_models
            .load_many(
                synonym::Entity::find().filter(synonym::Column::SynonymableType.eq("anime")),
                database,
            )
            .await?;

        let documents = models
            .into_iter()
            .zip(anime_models)
            .zip(synonyms)
            .zip(song_models)
            .map(|(((model, anime), synonyms), song)| {
                AnimeThemeDocument::from((model, anime, synonyms, song))
            })
            .collect();

        Ok(documents)
    })
}
