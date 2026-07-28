use sea_orm::entity::prelude::*;

use crate::entities::content::{audio, videoscript};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "videos")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "video_id")]
    pub id: u64,
    pub audio_id: Option<u64>,
    pub script_id: Option<u64>,
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
        from = "Column::ScriptId",
        to = "videoscript::Column::Id"
    )]
    VideoScript,
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

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Copy, Clone, Eq, EnumIter, PartialEq, DeriveActiveEnum)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum VideoOverlap {
    #[sea_orm(num_value = 0)]
    None,

    #[sea_orm(num_value = 1)]
    Trans,

    #[sea_orm(num_value = 2)]
    Over,
}

#[derive(Debug, Copy, Clone, Eq, EnumIter, PartialEq, DeriveActiveEnum)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum VideoSource {
    #[sea_orm(num_value = 0)]
    WEB,

    #[sea_orm(num_value = 1)]
    RAW,

    #[sea_orm(num_value = 2)]
    BD,

    #[sea_orm(num_value = 3)]
    DVD,

    #[sea_orm(num_value = 4)]
    VHS,

    #[sea_orm(num_value = 5)]
    LD,
}
