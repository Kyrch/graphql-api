use sea_orm::entity::prelude::*;

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
