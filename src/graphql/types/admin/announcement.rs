use async_graphql::{ComplexObject, SimpleObject, dataloader::DataLoader};
use chrono::{DateTime, Utc};

use crate::{entities::admin::announcement, graphql::utils::format_datetime};

/// Represents a site-wide message to be broadcasted on the homepage.
#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Announcement {
    /// The primary key of the resource
    pub id: u64,
    /// The announcement text
    pub content: String,
    #[graphql(skip)]
    pub start_at: Option<DateTime<Utc>>,
    #[graphql(skip)]
    pub end_at: Option<DateTime<Utc>>,
}

#[ComplexObject]
impl Announcement {
    /// The start date of the resource
    async fn start_at(&self, #[graphql(default = "%+")] format: String) -> Option<String> {
        format_datetime(self.start_at.as_ref(), &format)
    }

    /// The end date of the resource
    async fn end_at(&self, #[graphql(default = "%+")] format: String) -> Option<String> {
        format_datetime(self.end_at.as_ref(), &format)
    }
}

impl From<announcement::Model> for Announcement {
    fn from(model: announcement::Model) -> Self {
        Self {
            id: model.id,
            content: model.content,
            start_at: model.start_at,
            end_at: model.end_at,
        }
    }
}
