use sea_orm::EntityTrait;

pub mod admin;
pub mod auth;
pub mod content;
pub mod document;
pub mod list;

pub trait SoftDeleteEntity: EntityTrait {
    fn deleted_at_column() -> Self::Column;
}
