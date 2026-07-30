use chrono::Utc;
use sea_orm::entity::prelude::*;

use crate::entities::{SoftDeleteEntity, content::performance};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "artists")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "artist_id")]
    pub id: u64,
    pub slug: String,
    pub name: String,
    pub name_native: Option<String>,
    pub information: Option<String>,
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
    #[sea_orm(has_many = "performance::Entity")]
    Performances,
}

impl Related<performance::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Performances.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
