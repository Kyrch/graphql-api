use sea_orm::entity::prelude::*;

#[derive(Debug, Copy, Clone, Eq, EnumIter, PartialEq, DeriveActiveEnum)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum AnimeFormat {
    #[sea_orm(num_value = 0)]
    TV,

    #[sea_orm(num_value = 1)]
    TVShort,

    #[sea_orm(num_value = 2)]
    OVA,

    #[sea_orm(num_value = 3)]
    Movie,

    #[sea_orm(num_value = 4)]
    Special,

    #[sea_orm(num_value = 5)]
    ONA,
}
