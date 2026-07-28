use sea_orm::entity::prelude::*;

use crate::entities::animetheme::animetheme;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "groups")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "group_id")]
    pub id: u64,
    pub name: String,
    pub slug: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "animetheme::Entity")]
    AnimeThemes,
}

impl Related<animetheme::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AnimeThemes.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
