use sea_orm::entity::prelude::*;

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

#[derive(Debug, Copy, Clone, Eq, EnumIter, PartialEq, DeriveActiveEnum)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum PlaylistVisibility {
    #[sea_orm(num_value = 0)]
    Public,

    #[sea_orm(num_value = 1)]
    Private,

    #[sea_orm(num_value = 2)]
    Unlisted,
}
