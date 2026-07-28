use async_graphql::SimpleObject;

use crate::entities::series;

/// Represents a collection of related anime.
///
/// For example, the Monogatari series is the collection of the Bakemonogatari anime and its related productions.
#[derive(SimpleObject)]
pub struct Series {
    /// The primary title of the series
    pub title: String,
    /// The URL slug & route key of the resource
    pub slug: String,
}

impl From<series::Model> for Series {
    fn from(model: series::Model) -> Self {
        Self {
            slug: model.slug,
            title: model.title,
        }
    }
}
