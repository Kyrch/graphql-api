use async_graphql::{ComplexObject, SimpleObject};
use chrono::{DateTime, Utc};

use crate::graphql::utils::format_datetime;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct ImageEdgeFields {
    /// Used to sort the images
    pub depth: i32,

    #[graphql(skip)]
    pub created_at: Option<DateTime<Utc>>,

    #[graphql(skip)]
    pub updated_at: Option<DateTime<Utc>>,
}

#[ComplexObject]
impl ImageEdgeFields {
    /// The date that the resource was created
    async fn created_at(&self, #[graphql(default = "%+")] format: String) -> Option<String> {
        format_datetime(self.created_at.as_ref(), &format)
    }

    /// The date that the resource was updated
    async fn updated_at(&self, #[graphql(default = "%+")] format: String) -> Option<String> {
        format_datetime(self.updated_at.as_ref(), &format)
    }
}
