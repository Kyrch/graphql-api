use sea_orm::entity::prelude::*;

use crate::entities::anime;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "synonyms")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "synonym_id")]
    pub id: u64,
    pub language: Option<String>,
    pub synonymable_type: String,
    pub synonymable_id: u64,
    pub text: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "anime::Entity",
        from = "Column::SynonymableId",
        to = "anime::Column::Id"
    )]
    Anime,
}

impl Related<anime::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Anime.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
