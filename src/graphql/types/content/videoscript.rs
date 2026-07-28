use async_graphql::SimpleObject;

use crate::entities::content::videoscript;

/// Represents an encoding script used to produce a video.
///
/// For example, the 2009/Summer/Bakemonogatari-OP1.txt video script represents the encoding script of the Bakemonogatari-OP1.webm video.
#[derive(SimpleObject)]
pub struct VideoScript {
    #[graphql(skip)]
    pub id: u64,
    #[graphql(skip)]
    pub video_id: u64,
    /// The path of the file in storage
    pub path: String,
}

impl From<videoscript::Model> for VideoScript {
    fn from(model: videoscript::Model) -> Self {
        Self {
            id: model.id,
            video_id: model.video_id,
            path: model.path,
        }
    }
}
