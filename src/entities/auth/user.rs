use chrono::Utc;
use sea_orm::entity::prelude::*;

use crate::entities::list::playlist;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub name: String,
    pub email: String,
    pub password: String,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: Option<chrono::DateTime<Utc>>,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: Option<chrono::DateTime<Utc>>,

    #[sea_orm(has_many)]
    pub playlists: HasMany<playlist::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
