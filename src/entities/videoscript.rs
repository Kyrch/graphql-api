use sea_orm::entity::prelude::*;

use crate::entities::video;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "video_scripts")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "script_id")]
    pub id: u64,
    pub video_id: u64,
    pub path: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "video::Entity",
        from = "Column::VideoId",
        to = "video::Column::Id"
    )]
    Video,
}

impl Related<video::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Video.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
