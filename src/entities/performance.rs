use sea_orm::entity::prelude::*;

use crate::entities::{artist, song};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "performances")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "performance_id")]
    pub id: u64,
    pub alias: Option<String>,
    pub r#as: Option<String>,
    pub member_alias: Option<String>,
    pub member_as: Option<String>,
    pub relevance: i32,
    pub artist_id: u64,
    pub member_id: Option<u64>,
    pub song_id: u64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "artist::Entity",
        from = "Column::ArtistId",
        to = "artist::Column::Id"
    )]
    Artist,

    #[sea_orm(
        belongs_to = "artist::Entity",
        from = "Column::MemberId",
        to = "artist::Column::Id"
    )]
    Member,

    #[sea_orm(
        belongs_to = "song::Entity",
        from = "Column::SongId",
        to = "song::Column::Id"
    )]
    Song,
}

impl Entity {
    pub fn member() -> RelationDef {
        Relation::Member.def()
    }
}

impl Related<artist::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Artist.def()
    }
}

impl Related<song::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Song.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
