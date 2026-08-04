use chrono::Utc;
use sea_orm::{LinkDef, entity::prelude::*};

use crate::entities::{
    SoftDeleteEntity,
    content::{performance, synonym},
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "artists")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "artist_id")]
    pub id: u64,
    pub slug: String,
    pub name: String,
    pub name_native: Option<String>,
    pub information: Option<String>,
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
        has_many = "synonym::Entity",
        on_condition = r#"synonym::Column::SynonymableType.eq("artist")"#
    )]
    Synonyms,

    #[sea_orm(
        has_many = "performance::Entity",
        from = "Column::Id",
        to = "performance::Column::ArtistId"
    )]
    Performances,

    #[sea_orm(
        has_many = "performance::Entity",
        from = "Column::Id",
        to = "performance::Column::MemberId"
    )]
    MemberPerformances,
}

impl Related<synonym::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Synonyms.def()
    }
}

impl Related<performance::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Performances.def()
    }
}

pub struct ArtistPerformances;

impl Linked for ArtistPerformances {
    type FromEntity = Entity;
    type ToEntity = performance::Entity;

    fn link(&self) -> Vec<LinkDef> {
        vec![Relation::Performances.def().into()]
    }
}

pub struct MemberPerformances;

impl Linked for MemberPerformances {
    type FromEntity = Entity;
    type ToEntity = performance::Entity;

    fn link(&self) -> Vec<LinkDef> {
        vec![Relation::MemberPerformances.def().into()]
    }
}

impl ActiveModelBehavior for ActiveModel {}
