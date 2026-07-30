use chrono::Utc;
use sea_orm::entity::prelude::*;

use crate::entities::{
    SoftDeleteEntity,
    content::{anime, anime_series},
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "series")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "series_id")]
    pub id: u64,
    pub slug: String,
    pub title: String,
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
    #[sea_orm(has_many = "anime_series::Entity")]
    AnimeSeries,
}

impl Related<anime::Entity> for Entity {
    fn to() -> RelationDef {
        anime_series::Relation::Anime.def()
    }

    fn via() -> Option<RelationDef> {
        Some(anime_series::Relation::Series.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
