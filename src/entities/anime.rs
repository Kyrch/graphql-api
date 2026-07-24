use sea_orm::{
    entity::prelude::*,
    EntityTrait,
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "anime")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "anime_id")]
    pub id: u64,
    pub format: i32,
    pub season: i32,
    pub slug: String,
    pub title: String,
    pub title_english: Option<String>,
    pub title_native: Option<String>,
    pub year: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}