use sea_orm::entity::prelude::*;

use crate::entities::video;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "audios")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "audio_id")]
    pub id: u64,
    pub basename: String,
    pub filename: String,
    pub mimetype: String,
    pub path: String,
    pub size: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "video::Entity")]
    Videos,
}

impl Related<video::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Videos.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
