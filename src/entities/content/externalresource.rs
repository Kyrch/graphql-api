use sea_orm::entity::prelude::*;

use crate::enums::content::resourcesite::ResourceSite;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "resources")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "resource_id")]
    pub id: u64,
    pub external_id: Option<i32>,
    pub link: String,
    pub site: ResourceSite,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
