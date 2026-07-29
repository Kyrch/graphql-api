use sea_orm::entity::prelude::*;

use crate::{
    entities::content::{
        anime_series, anime_studios, animetheme::animetheme, series, studio, synonym,
    },
    enums::content::{animeformat::AnimeFormat, animeseason::AnimeSeason},
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "anime")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "anime_id")]
    pub id: u64,
    pub format: Option<AnimeFormat>,
    pub season: Option<AnimeSeason>,
    pub slug: String,
    pub synopsis: Option<String>,
    pub title: String,
    pub title_english: Option<String>,
    pub title_native: Option<String>,
    pub year: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        has_many = "synonym::Entity",
        on_condition = r#"synonym::Column::SynonymableType.eq("anime")"#
    )]
    Synonyms,

    #[sea_orm(has_many = "animetheme::Entity")]
    AnimeThemes,

    #[sea_orm(has_many = "anime_series::Entity")]
    AnimeSeries,

    #[sea_orm(has_many = "anime_studios::Entity")]
    AnimeStudio,
}

impl Related<synonym::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Synonyms.def()
    }
}

impl Related<animetheme::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AnimeThemes.def()
    }
}

impl Related<series::Entity> for Entity {
    fn to() -> RelationDef {
        anime_series::Relation::Series.def()
    }

    fn via() -> Option<RelationDef> {
        Some(anime_series::Relation::Anime.def().rev())
    }
}

impl Related<studio::Entity> for Entity {
    fn to() -> RelationDef {
        anime_studios::Relation::Studio.def()
    }

    fn via() -> Option<RelationDef> {
        Some(anime_studios::Relation::Anime.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
