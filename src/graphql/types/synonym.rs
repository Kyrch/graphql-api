use async_graphql::SimpleObject;

use crate::entities::synonym;

#[derive(SimpleObject)]
pub struct Synonym {
    pub language: Option<String>,
    pub text: String,
}

impl From<synonym::Model> for Synonym {
    fn from(model: synonym::Model) -> Self {
        Self {
            language: model.language,
            text: model.text,
        }
    }
}
