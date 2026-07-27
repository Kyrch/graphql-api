use sea_orm::entity::prelude::*;

use crate::entities::anime;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "anime_themes")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "theme_id")]
    pub id: u64,
    pub anime_id: u64,
    pub group_id: Option<u64>,
    pub sequence: Option<i32>,
    pub slug: String,
    pub song_id: Option<u64>,
    #[sea_orm(column_name = "type")]
    pub themetype: ThemeType,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "anime::Entity",
        from = "Column::AnimeId",
        to = "anime::Column::Id"
    )]
    Anime,
}

impl Related<anime::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Anime.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Copy, Clone, Eq, EnumIter, PartialEq, DeriveActiveEnum)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum ThemeType {
    #[sea_orm(num_value = 0)]
    OP,

    #[sea_orm(num_value = 1)]
    ED,

    #[sea_orm(num_value = 2)]
    IN,
}
