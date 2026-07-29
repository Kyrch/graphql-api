use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "pages")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "page_id")]
    pub id: u64,
    pub name: String,
    pub slug: String,
    pub body: String,
    pub previous_id: Option<u64>,
    pub next_id: Option<u64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
