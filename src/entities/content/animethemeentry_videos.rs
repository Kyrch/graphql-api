use sea_orm::{
    entity::prelude::*,
    sqlx::types::chrono::{self, Utc},
};

use crate::entities::content::{animetheme::animethemeentry::animethemeentry, video};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "anime_theme_entry_video")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub entry_id: u64,
    #[sea_orm(primary_key)]
    pub video_id: u64,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: Option<chrono::DateTime<Utc>>,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: Option<chrono::DateTime<Utc>>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "animethemeentry::Entity",
        from = "Column::EntryId",
        to = "animethemeentry::Column::Id"
    )]
    AnimeThemeEntry,

    #[sea_orm(
        belongs_to = "video::Entity",
        from = "Column::VideoId",
        to = "video::Column::Id"
    )]
    Video,
}

impl Related<animethemeentry::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AnimeThemeEntry.def()
    }
}

impl Related<video::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Video.def()
    }
}
