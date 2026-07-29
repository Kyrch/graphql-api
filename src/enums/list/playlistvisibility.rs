use sea_orm::{DeriveActiveEnum, EnumIter};

#[derive(Debug, Copy, Clone, Eq, EnumIter, PartialEq, DeriveActiveEnum)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum PlaylistVisibility {
    #[sea_orm(num_value = 0)]
    Public,

    #[sea_orm(num_value = 1)]
    Private,

    #[sea_orm(num_value = 2)]
    Unlisted,
}
