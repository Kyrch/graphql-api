use chrono::Utc;
use sea_orm::entity::prelude::*;

use crate::entities::{SoftDeleteEntity, content::video};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "video_scripts")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "script_id")]
    pub id: u64,
    pub video_id: u64,
    pub path: String,
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
    #[sea_orm(
        belongs_to = "video::Entity",
        from = "Column::VideoId",
        to = "video::Column::Id"
    )]
    Video,
}

impl Related<video::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Video.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
