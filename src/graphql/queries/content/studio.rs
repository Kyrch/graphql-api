use async_graphql::{Context, Object, Result};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{entities::content::studio, graphql::types::content::studio::Studio};

#[derive(Default)]
pub struct StudioQuery;

#[Object]
impl StudioQuery {
    async fn studio(&self, ctx: &Context<'_>, slug: String) -> Result<Option<Studio>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let studio: Option<studio::Model> = studio::Entity::find()
            .filter(studio::Column::Slug.eq(slug))
            .one(db)
            .await?;

        Ok(studio.map(|a| a.into()))
    }
}
