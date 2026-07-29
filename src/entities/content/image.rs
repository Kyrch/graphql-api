use sea_orm::entity::prelude::*;

use crate::enums::content::imagefacet::ImageFacet;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "images")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "image_id")]
    pub id: u64,
    pub facet: ImageFacet,
    pub path: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
