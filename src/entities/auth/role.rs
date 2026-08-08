use chrono::Utc;
use sea_orm::entity::prelude::*;

use crate::entities::auth::user;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "roles")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub name: String,
    pub priority: i32,
    pub color: Option<String>,
    pub default: bool,
    pub guard_name: String,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: Option<chrono::DateTime<Utc>>,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: Option<chrono::DateTime<Utc>>,

    #[sea_orm(has_many, via = "model_has_roles")]
    pub users: HasMany<user::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
