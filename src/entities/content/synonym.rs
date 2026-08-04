use chrono::Utc;
use sea_orm::entity::prelude::*;

use crate::entities::{
    SoftDeleteEntity,
    content::{anime, artist},
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "synonyms")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "synonym_id")]
    pub id: u64,
    pub language: Option<String>,
    pub synonymable_type: String,
    pub synonymable_id: u64,
    pub text: String,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: Option<chrono::DateTime<Utc>>,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: Option<chrono::DateTime<Utc>>,
    #[sea_orm(column_type = "Timestamp")]
    pub deleted_at: Option<chrono::DateTime<Utc>>,
}

impl SoftDeleteEntity for Entity {
    fn deleted_at_column() -> Self::Column {
        Column::DeletedAt
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "anime::Entity",
        from = "Column::SynonymableId",
        to = "anime::Column::Id"
    )]
    Anime,

    #[sea_orm(
        belongs_to = "artist::Entity",
        from = "Column::SynonymableId",
        to = "artist::Column::Id"
    )]
    Artist,
}

impl Related<anime::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Anime.def()
    }
}

impl Related<artist::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Artist.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
