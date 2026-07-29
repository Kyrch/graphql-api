use sea_orm::entity::prelude::*;

use crate::enums::list::playlistvisibility::PlaylistVisibility;

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
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
