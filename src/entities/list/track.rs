use chrono::Utc;
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
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: Option<chrono::DateTime<Utc>>,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: Option<chrono::DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
