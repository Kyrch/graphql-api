use chrono::Utc;
use sea_orm::{entity::prelude::*};

use crate::entities::content::image;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "imageables")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub depth: i32,
    pub image_id: u64,
    pub imageable_type: String,
    pub imageable_id: u64,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: Option<chrono::DateTime<Utc>>,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: Option<chrono::DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "image::Entity",
        from = "Column::ImageId",
        to = "image::Column::Id"
    )]
    Image,
}

impl Related<image::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Image.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
