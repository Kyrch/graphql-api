use sea_orm::entity::prelude::*;

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

#[derive(Debug, Copy, Clone, Eq, EnumIter, PartialEq, DeriveActiveEnum)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum ImageFacet {
    #[sea_orm(num_value = 0)]
    SmallCover,

    #[sea_orm(num_value = 1)]
    LargeCover,

    #[sea_orm(num_value = 2)]
    Grill,

    #[sea_orm(num_value = 3)]
    Document,

    #[sea_orm(num_value = 4)]
    Avatar,

    #[sea_orm(num_value = 5)]
    Banner,
}
