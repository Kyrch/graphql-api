use async_graphql::SimpleObject;

use crate::entities::song;

#[derive(SimpleObject)]
pub struct SongTitle {
    /// The romaji title of the composition
    romaji: Option<String>,
    /// The native title of the composition
    native: Option<String>,
}

impl From<&song::Model> for SongTitle {
    fn from(model: &song::Model) -> Self {
        Self {
            romaji: model.title.clone(),
            native: model.title_native.clone(),
        }
    }
}

/// Represents the composition that accompanies an AnimeTheme.
///
/// For example, Staple Stable is the song for the Bakemonogatari OP1 AnimeTheme.
#[derive(SimpleObject)]
pub struct Song {
    /// The title of the composition
    pub title: SongTitle,
}

impl From<song::Model> for Song {
    fn from(model: song::Model) -> Self {
        let title = SongTitle::from(&model);
        Self { title }
    }
}
