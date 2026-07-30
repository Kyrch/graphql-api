use async_graphql::{Context, Object, Result};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
    entities::content::series, graphql::types::content::series::Series, scopes::without_trashed,
};

#[derive(Default)]
pub struct SeriesQuery;

#[Object]
impl SeriesQuery {
    async fn series(&self, ctx: &Context<'_>, slug: String) -> Result<Option<Series>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let series = series::Entity::find()
            .filter(without_trashed::<series::Entity>())
            .filter(series::Column::Slug.eq(slug))
            .one(db)
            .await?;

        Ok(series.map(|a| a.into()))
    }
}
