use async_graphql::Enum;

use crate::enums::content::videooverlap::VideoOverlap as VideoOverlapEnum;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum VideoOverlap {
    None,
    Trans,
    Over,
}

impl From<VideoOverlapEnum> for VideoOverlap {
    fn from(value: VideoOverlapEnum) -> Self {
        match value {
            VideoOverlapEnum::None => VideoOverlap::None,
            VideoOverlapEnum::Trans => VideoOverlap::Trans,
            VideoOverlapEnum::Over => VideoOverlap::Over,
        }
    }
}
