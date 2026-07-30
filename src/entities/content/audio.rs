use chrono::Utc;
use sea_orm::entity::prelude::*;

use crate::entities::{SoftDeleteEntity, content::video};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "audios")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "audio_id")]
    pub id: u64,
    pub basename: String,
    pub filename: String,
    pub mimetype: String,
    pub path: String,
    pub size: i32,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: Option<chrono::DateTime<Utc>>,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: Option<chrono::DateTime<Utc>>,
    #[sea_orm(column_type = "Timestamp")]
    pub deleted_at: Option<chrono::DateTime<Utc>>,
}

impl SoftDeleteEntity for Entity {
    fn deleted_at_column() -> Self::Column {
        Column::DeletedAt
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "video::Entity")]
    Videos,
}

impl Related<video::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Videos.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
