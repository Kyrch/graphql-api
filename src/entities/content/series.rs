use sea_orm::entity::prelude::*;

use crate::entities::content::{anime, anime_series};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "series")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "series_id")]
    pub id: u64,
    pub slug: String,
    pub title: String,
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
