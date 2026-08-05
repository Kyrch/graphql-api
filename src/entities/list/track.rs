use chrono::Utc;
use sea_orm::entity::prelude::*;

use crate::entities::{
    content::{animetheme::animethemeentry::animethemeentry, video},
    list::playlist,
};

#[sea_orm::model]
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

    #[sea_orm(belongs_to, from = "playlist_id", to = "id")]
    pub playlist: BelongsTo<playlist::Entity>,

    #[sea_orm(self_ref, relation_enum = "Previous", from = "previous_id", to = "id")]
    pub previous: BelongsTo<Option<Entity>>,

    #[sea_orm(self_ref, relation_enum = "Next", from = "next_id", to = "id")]
    pub next: BelongsTo<Option<Entity>>,

    #[sea_orm(belongs_to, from = "entry_id", to = "id")]
    pub entry: BelongsTo<Option<animethemeentry::Entity>>,

    #[sea_orm(belongs_to, from = "video_id", to = "id")]
    pub video: BelongsTo<Option<video::Entity>>,
}

impl ActiveModelBehavior for ActiveModel {}
