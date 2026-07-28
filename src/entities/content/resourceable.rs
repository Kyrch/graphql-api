use chrono::Utc;
use sea_orm::entity::prelude::*;

use crate::entities::content::externalresource;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "resourceables")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub r#as: Option<String>,
    pub resource_id: u64,
    pub resourceable_type: String,
    pub resourceable_id: u64,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: Option<chrono::DateTime<Utc>>,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: Option<chrono::DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "externalresource::Entity",
        from = "Column::ResourceId",
        to = "externalresource::Column::Id"
    )]
    Resource,
}

impl Related<externalresource::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Resource.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
