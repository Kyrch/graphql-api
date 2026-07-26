use sea_orm::{EntityTrait, entity::prelude::*};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "anime")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "anime_id")]
    pub id: u64,
    pub format: AnimeFormat,
    pub season: AnimeSeason,
    pub slug: String,
    pub synopsis: Option<String>,
    pub title: String,
    pub title_english: Option<String>,
    pub title_native: Option<String>,
    pub year: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Copy, Clone, Eq, EnumIter, PartialEq, DeriveActiveEnum)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum AnimeFormat {
    #[sea_orm(num_value = 0)]
    TV,

    #[sea_orm(num_value = 1)]
    TVShort,

    #[sea_orm(num_value = 2)]
    OVA,

    #[sea_orm(num_value = 3)]
    Movie,

    #[sea_orm(num_value = 4)]
    Special,

    #[sea_orm(num_value = 5)]
    ONA,
}

#[derive(Debug, Copy, Clone, Eq, EnumIter, PartialEq, DeriveActiveEnum)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum AnimeSeason {
    #[sea_orm(num_value = 0)]
    Winter,

    #[sea_orm(num_value = 1)]
    Spring,

    #[sea_orm(num_value = 2)]
    Summer,

    #[sea_orm(num_value = 3)]
    Fall,
}
