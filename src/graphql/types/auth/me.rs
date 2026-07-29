use async_graphql::SimpleObject;

use crate::entities::auth::user;

/// Represents an AnimeThemes account.
#[derive(SimpleObject)]
pub struct Me {
    /// The primary key of the resource
    pub id: u64,
    /// The username of the resource
    pub name: String,
}

impl From<user::Model> for Me {
    fn from(model: user::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
        }
    }
}
