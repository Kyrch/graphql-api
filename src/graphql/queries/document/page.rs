use async_graphql::{Context, Object, Result};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{entities::document::page, graphql::types::document::page::Page};

#[derive(Default)]
pub struct PageQuery;

#[Object]
impl PageQuery {
    async fn page(&self, ctx: &Context<'_>, slug: String) -> Result<Option<Page>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let page = page::Entity::find()
            .filter(page::Column::Slug.eq(slug))
            .one(db)
            .await?;

        Ok(page.map(|a| a.into()))
    }
}
