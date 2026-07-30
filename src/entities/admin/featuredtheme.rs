use chrono::Utc;
use sea_orm::entity::prelude::*;

use crate::entities::{
    auth::user,
    content::{animetheme::animethemeentry::animethemeentry, video},
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "featured_themes")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "featured_theme_id")]
    pub id: u64,
    #[sea_orm(column_type = "Timestamp")]
    pub start_at: Option<chrono::DateTime<Utc>>,
    #[sea_orm(column_type = "Timestamp")]
    pub end_at: Option<chrono::DateTime<Utc>>,
    pub entry_id: Option<u64>,
    pub video_id: Option<u64>,
    pub user_id: Option<u64>,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: Option<chrono::DateTime<Utc>>,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: Option<chrono::DateTime<Utc>>,
}

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

    #[sea_orm(
        belongs_to = "user::Entity",
        from = "Column::UserId",
        to = "user::Column::Id"
    )]
    User,
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

impl Related<user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
