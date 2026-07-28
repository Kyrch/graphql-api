use sea_orm::entity::prelude::*;

use crate::entities::performance;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "artists")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "artist_id")]
    pub id: u64,
    pub slug: String,
    pub name: String,
    pub name_native: Option<String>,
    pub information: Option<String>,
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
