use async_graphql::SimpleObject;

use crate::entities::animetheme::animethemeentry::animethemeentry;

#[derive(SimpleObject)]
pub struct AnimeThemeEntry {
    pub episodes: Option<String>,
    pub likes_count: i32,
    pub notes: Option<String>,
    pub nsfw: bool,
    pub spoiler: bool,
    pub tracks_count: i32,
    pub version: i32,
}

impl From<animethemeentry::Model> for AnimeThemeEntry {
    fn from(model: animethemeentry::Model) -> Self {
        Self {
            episodes: model.episodes,
            likes_count: model.likes_count,
            notes: model.notes,
            nsfw: model.nsfw,
            spoiler: model.spoiler,
            tracks_count: model.tracks_count,
            version: model.version,
        }
    }
}
