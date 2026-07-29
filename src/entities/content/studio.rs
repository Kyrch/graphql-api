use sea_orm::entity::prelude::*;

use crate::entities::content::{anime, anime_studios};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "studios")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "studio_id")]
    pub id: u64,
    pub slug: String,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "anime_studios::Entity")]
    AnimeStudio,
}

impl Related<anime::Entity> for Entity {
    fn to() -> RelationDef {
        anime_studios::Relation::Anime.def()
    }

    fn via() -> Option<RelationDef> {
        Some(anime_studios::Relation::Studio.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
