use chrono::Utc;
use sea_orm::entity::prelude::*;

use crate::entities::{
    SoftDeleteEntity,
    content::{animetheme::animetheme, animethemeentry_videos, video},
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "anime_theme_entries")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "entry_id")]
    pub id: u64,
    pub theme_id: u64,
    pub episodes: Option<String>,
    pub likes_count: i32,
    pub notes: Option<String>,
    pub nsfw: bool,
    pub spoiler: bool,
    pub tracks_count: i32,
    pub version: i32,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: Option<chrono::DateTime<Utc>>,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: Option<chrono::DateTime<Utc>>,
    #[sea_orm(column_type = "Timestamp")]
    pub deleted_at: Option<chrono::DateTime<Utc>>,
}

impl SoftDeleteEntity for Entity {
    fn deleted_at_column() -> Self::Column {
        Column::DeletedAt
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "animetheme::Entity",
        from = "Column::ThemeId",
        to = "animetheme::Column::Id"
    )]
    AnimeTheme,

    #[sea_orm(has_many = "animethemeentry_videos::Entity")]
    AnimeThemeEntryVideos,
}

impl Related<animetheme::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AnimeTheme.def()
    }
}

impl Related<video::Entity> for Entity {
    fn to() -> RelationDef {
        animethemeentry_videos::Relation::Video.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            animethemeentry_videos::Relation::AnimeThemeEntry
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}
