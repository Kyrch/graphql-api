use sea_orm::entity::prelude::*;

#[derive(Debug, Copy, Clone, Eq, EnumIter, PartialEq, DeriveActiveEnum)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum AnimeSeason {
    #[sea_orm(num_value = 0)]
    Winter,

    #[sea_orm(num_value = 1)]
    Spring,

    #[sea_orm(num_value = 2)]
    Summer,

    #[sea_orm(num_value = 3)]
    Fall,
}
