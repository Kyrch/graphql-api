use chrono::Utc;
use sea_orm::entity::prelude::*;

use crate::{
    entities::{
        SoftDeleteEntity,
        content::{
            animetheme::animethemeentry::animethemeentry, animethemeentry_videos, audio,
            videoscript,
        },
    },
    enums::{
        LocalizedEnum,
        content::{videooverlap::VideoOverlap, videosource::VideoSource},
    },
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "videos")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "video_id")]
    pub id: u64,
    pub audio_id: Option<u64>,
    pub basename: String,
    pub filename: String,
    pub lyrics: bool,
    pub mimetype: String,
    pub nc: bool,
    pub overlap: VideoOverlap,
    pub path: String,
    pub resolution: Option<i32>,
    pub size: Option<i32>,
    pub source: Option<VideoSource>,
    pub subbed: bool,
    pub uncen: bool,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: Option<chrono::DateTime<Utc>>,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: Option<chrono::DateTime<Utc>>,
    #[sea_orm(column_type = "Timestamp")]
    pub deleted_at: Option<chrono::DateTime<Utc>>,
}

impl Model {
    pub fn tags(&self) -> String {
        let mut tags: Vec<String> = Vec::new();

        if self.nc {
            tags.push("NC".to_string());
        }

        if let Some(source) = self.source {
            if matches!(source, VideoSource::BD | VideoSource::DVD) {
                tags.push(source.localize().to_string());
            }
        }

        if let Some(resolution) = self.resolution {
            if resolution != 720 {
                tags.push(resolution.to_string());
            }
        }

        if self.subbed {
            tags.push("Subbed".to_string());
        } else if self.lyrics {
            tags.push("Lyrics".to_string());
        }

        tags.join("")
    }
}

impl SoftDeleteEntity for Entity {
    fn deleted_at_column() -> Self::Column {
        Column::DeletedAt
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "audio::Entity",
        from = "Column::AudioId",
        to = "audio::Column::Id"
    )]
    Audio,

    #[sea_orm(
        belongs_to = "videoscript::Entity",
        from = "Column::Id",
        to = "videoscript::Column::VideoId"
    )]
    VideoScript,

    #[sea_orm(has_many = "animethemeentry_videos::Entity")]
    AnimeThemeEntryVideos,
}

impl Related<audio::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Audio.def()
    }
}

impl Related<videoscript::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::VideoScript.def()
    }
}

impl Related<animethemeentry::Entity> for Entity {
    fn to() -> RelationDef {
        animethemeentry_videos::Relation::AnimeThemeEntry.def()
    }

    fn via() -> Option<RelationDef> {
        Some(animethemeentry_videos::Relation::Video.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
