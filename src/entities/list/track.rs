use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "playlist_tracks")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "track_id")]
    pub id: u64,
    pub entry_id: Option<u64>,
    pub next_id: Option<u64>,
    pub playlist_id: u64,
    pub position: i32,
    pub previous_id: Option<u64>,
    pub video_id: Option<u64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
