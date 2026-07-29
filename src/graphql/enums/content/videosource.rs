use async_graphql::Enum;

use crate::enums::content::videosource::VideoSource as VideoSourceEnum;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum VideoSource {
    WEB,
    RAW,
    BD,
    DVD,
    VHS,
    LD,
}

impl From<VideoSourceEnum> for VideoSource {
    fn from(value: VideoSourceEnum) -> Self {
        match value {
            VideoSourceEnum::WEB => VideoSource::WEB,
            VideoSourceEnum::RAW => VideoSource::RAW,
            VideoSourceEnum::BD => VideoSource::BD,
            VideoSourceEnum::DVD => VideoSource::DVD,
            VideoSourceEnum::VHS => VideoSource::VHS,
            VideoSourceEnum::LD => VideoSource::LD,
        }
    }
}
