use chrono::Utc;
use sea_orm::entity::prelude::*;

use crate::{
    entities::{auth::user, list::track},
    enums::list::playlistvisibility::PlaylistVisibility,
};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "playlists")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "playlist_id")]
    pub id: u64,
    pub hashid: Option<String>,
    pub description: Option<String>,
    pub first_id: Option<u64>,
    pub last_id: Option<u64>,
    pub name: String,
    pub user_id: Option<u64>,
    pub visibility: PlaylistVisibility,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: Option<chrono::DateTime<Utc>>,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: Option<chrono::DateTime<Utc>>,

    #[sea_orm(belongs_to, relation_enum = "First", from = "first_id", to = "id")]
    pub first: BelongsTo<Option<track::Entity>>,

    #[sea_orm(belongs_to, relation_enum = "Last", from = "last_id", to = "id")]
    pub last: BelongsTo<Option<track::Entity>>,

    #[sea_orm(belongs_to, from = "user_id", to = "id")]
    pub user: BelongsTo<Option<user::Entity>>,

    #[sea_orm(has_many, relation_enum = "Tracks")]
    pub tracks: HasMany<track::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
