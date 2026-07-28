use async_graphql::SimpleObject;

use crate::entities::series;

#[derive(SimpleObject)]
pub struct Series {
    pub slug: String,
    pub title: String,
}

impl From<series::Model> for Series {
    fn from(model: series::Model) -> Self {
        Self {
            slug: model.slug,
            title: model.title,
        }
    }
}
