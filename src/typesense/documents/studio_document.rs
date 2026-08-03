use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use typesense::Typesense;

use crate::{entities::content::studio, typesense::index_document::BuildDocumentsFuture};

#[derive(Debug, Clone, Serialize, Deserialize, Typesense)]
#[typesense(collection_name = "studios")]
pub struct StudioDocument {
    pub id: String,
    #[typesense(sort)]
    pub name: String,
    pub created_at: Option<i64>,
}

impl From<studio::Model> for StudioDocument {
    fn from(model: studio::Model) -> Self {
        Self {
            id: model.id.to_string(),
            name: model.name.clone(),
            created_at: model.created_at.map(|c| c.timestamp()),
        }
    }
}

pub fn build_studio_documents<'a>(
    models: Vec<studio::Model>,
    _database: &'a DatabaseConnection,
) -> BuildDocumentsFuture<'a, StudioDocument> {
    Box::pin(async move {
        let documents = models.into_iter().map(StudioDocument::from).collect();

        Ok(documents)
    })
}
