use sea_orm::entity::prelude::*;

use crate::entities::content::animetheme::animetheme;

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
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "animetheme::Entity",
        from = "Column::ThemeId",
        to = "animetheme::Column::Id"
    )]
    AnimeTheme,
}

impl Related<animetheme::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AnimeTheme.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
