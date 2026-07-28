use async_graphql::{ComplexObject, SimpleObject};
use chrono::{DateTime, Utc};

use crate::graphql::utils::format_datetime;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct ExternalResourceEdgeFields {
    /// Used to distinguish resources that map to the same resourceable
    pub r#as: Option<String>,

    #[graphql(skip)]
    pub created_at: Option<DateTime<Utc>>,

    #[graphql(skip)]
    pub updated_at: Option<DateTime<Utc>>,
}

#[ComplexObject]
impl ExternalResourceEdgeFields {
    /// The date that the resource was created
    async fn created_at(&self, #[graphql(default = "%+")] format: String) -> Option<String> {
        format_datetime(self.created_at.as_ref(), &format)
    }

    /// The date that the resource was updated
    async fn updated_at(&self, #[graphql(default = "%+")] format: String) -> Option<String> {
        format_datetime(self.updated_at.as_ref(), &format)
    }
}
