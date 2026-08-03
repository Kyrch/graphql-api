use anyhow::Result;
use async_graphql::Error;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Select};
use typesense::{models::SearchParameters, prelude::Document};

use crate::typesense::{client::TypesenseClient, documents::HasId};

pub async fn search<E, D>(
    db: &DatabaseConnection,
    typesense: &TypesenseClient,
    builder: Select<E>,
    id_column: E::Column,
    search: String,
    query_by: &str,
    query_by_weights: &str,
) -> Result<Vec<E::Model>>
where
    E: EntityTrait,
    D: Document + HasId,
{
    let documents = typesense
        .collection::<D>()
        .documents()
        .search(
            SearchParameters::builder()
                .q(search)
                .query_by(query_by)
                .query_by_weights(query_by_weights)
                .build(),
        )
        .await
        .map_err(|error| Error::new(error.to_string()))
        .unwrap();

    let ids: Vec<String> = documents
        .hits
        .unwrap_or_default()
        .into_iter()
        .filter_map(|hit| hit.document)
        .filter_map(|document| document.id().parse::<String>().ok())
        .collect();

    if ids.is_empty() {
        return Ok(Vec::new());
    }

    let models = builder.filter(id_column.is_in(ids)).all(db).await?;

    Ok(models.into_iter().map(|m| m.into()).collect())
}
