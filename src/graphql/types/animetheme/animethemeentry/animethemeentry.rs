use async_graphql::SimpleObject;

use crate::entities::animetheme::animethemeentry::animethemeentry;

/// Represents a version of an anime theme.
///
/// For example, the ED theme of the Bakemonogatari anime has three anime theme entries to represent three versions.
#[derive(SimpleObject)]
pub struct AnimeThemeEntry {
    /// The episodes that the theme is used for
    pub episodes: Option<String>,
    /// The number of likes recorded for the resource
    pub likes_count: i32,
    /// Any additional information for this sequence
    pub notes: Option<String>,
    /// Is not safe for work content included?
    pub nsfw: bool,
    /// Is content included that may spoil the viewer?
    pub spoiler: bool,
    /// The number of tracks belonging to the resource
    pub tracks_count: i32,
    /// The version number of the theme
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
