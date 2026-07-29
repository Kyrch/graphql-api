use sea_orm::{
    entity::prelude::*,
    sqlx::types::chrono::{self, Utc},
};

use crate::entities::content::{anime, studio};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "anime_studio")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub anime_id: u64,
    #[sea_orm(primary_key)]
    pub studio_id: u64,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: Option<chrono::DateTime<Utc>>,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: Option<chrono::DateTime<Utc>>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "anime::Entity",
        from = "Column::AnimeId",
        to = "anime::Column::Id"
    )]
    Anime,

    #[sea_orm(
        belongs_to = "studio::Entity",
        from = "Column::StudioId",
        to = "studio::Column::Id"
    )]
    Studio,
}

impl Related<anime::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Anime.def()
    }
}

impl Related<studio::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Studio.def()
    }
}
