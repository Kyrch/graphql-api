use sea_orm::entity::prelude::*;

#[derive(Debug, Copy, Clone, Eq, EnumIter, PartialEq, DeriveActiveEnum)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum ThemeType {
    #[sea_orm(num_value = 0)]
    OP,

    #[sea_orm(num_value = 1)]
    ED,

    #[sea_orm(num_value = 2)]
    IN,
}
