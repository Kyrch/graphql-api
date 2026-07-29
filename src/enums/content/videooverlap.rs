use sea_orm::entity::prelude::*;

#[derive(Debug, Copy, Clone, Eq, EnumIter, PartialEq, DeriveActiveEnum)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum VideoOverlap {
    #[sea_orm(num_value = 0)]
    None,

    #[sea_orm(num_value = 1)]
    Trans,

    #[sea_orm(num_value = 2)]
    Over,
}
