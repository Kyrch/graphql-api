use sea_orm::{
    entity::prelude::*,
    sqlx::types::chrono::{self, Utc},
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "anime_series")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub anime_id: u64,
    #[sea_orm(primary_key)]
    pub series_id: u64,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: Option<chrono::DateTime<Utc>>,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: Option<chrono::DateTime<Utc>>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::anime::Entity",
        from = "Column::AnimeId",
        to = "super::anime::Column::Id"
    )]
    Anime,

    #[sea_orm(
        belongs_to = "super::series::Entity",
        from = "Column::SeriesId",
        to = "super::series::Column::Id"
    )]
    Series,
}

impl Related<super::anime::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Anime.def()
    }
}

impl Related<super::series::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Series.def()
    }
}
