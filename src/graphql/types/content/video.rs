use async_graphql::{ComplexObject, Context, Enum, Result, SimpleObject, dataloader::DataLoader};

use crate::{
    entities::content::video::{
        self, VideoOverlap as VideoOverlapEnum, VideoSource as VideoSourceEnum,
    },
    graphql::{
        loaders::video::{video_audio::VideoAudioLoader, video_script::VideoScriptLoader},
        types::content::{audio::Audio, videoscript::VideoScript},
    },
};

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

/// Represents a WebM of an anime theme.
///
/// For example, the video Bakemonogatari-OP1.webm represents the WebM of the Bakemonogatari OP1 theme.
#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Video {
    #[graphql(skip)]
    pub id: u64,
    #[graphql(skip)]
    pub audio_id: Option<u64>,
    /// The basename of the file in storage
    pub basename: String,
    /// The filename of the file in storage
    pub filename: String,
    /// Does the video include subtitles of song lyrics?
    pub lyrics: bool,
    /// The media type of the file in storage
    pub mimetype: String,
    /// Is the video creditless?
    pub nc: bool,
    /// The degree to which the sequence and episode content overlap
    pub overlap: VideoOverlap,
    /// The path of the file in storage
    pub path: String,
    /// The frame height of the file in storage
    pub resolution: Option<i32>,
    /// The size of the file in storage in Bytes
    pub size: Option<i32>,
    /// Where did this video come from?
    pub source: Option<VideoSource>,
    /// Does the video include subtitles of dialogue?
    pub subbed: bool,
    /// Is the video an uncensored version of a censored sequence?
    pub uncen: bool,
}

#[ComplexObject]
impl Video {
    async fn audio(&self, ctx: &Context<'_>) -> Result<Option<Audio>> {
        let Some(audio_id) = self.audio_id else {
            return Ok(None);
        };

        let loader = ctx.data::<DataLoader<VideoAudioLoader>>()?;

        Ok(loader.load_one(audio_id).await?.map(Into::into))
    }

    async fn script(&self, ctx: &Context<'_>) -> Result<Option<VideoScript>> {
        let loader = ctx.data::<DataLoader<VideoScriptLoader>>()?;

        let script = loader.load_one(self.id).await?;

        Ok(script.map(Into::into))
    }
}

impl From<video::Model> for Video {
    fn from(model: video::Model) -> Self {
        Self {
            id: model.id,
            audio_id: model.audio_id,
            basename: model.basename,
            filename: model.filename,
            lyrics: model.lyrics,
            mimetype: model.mimetype,
            nc: model.nc,
            overlap: model.overlap.into(),
            path: model.path,
            resolution: model.resolution,
            size: model.size,
            source: model.source.map(Into::into),
            subbed: model.subbed,
            uncen: model.uncen,
        }
    }
}
