use async_graphql::{Context, Object, Result};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{entities::admin::featuredtheme, graphql::types::admin::featuredtheme::FeaturedTheme};

#[derive(Default)]
pub struct FeaturedThemeQuery;

#[Object]
impl FeaturedThemeQuery {
    async fn current_featured_theme(&self, ctx: &Context<'_>) -> Result<Option<FeaturedTheme>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let featured_theme = featuredtheme::Entity::find()
            .filter(featuredtheme::Column::StartAt.lte(chrono::Utc::now()))
            .filter(featuredtheme::Column::EndAt.gte(chrono::Utc::now()))
            .one(db)
            .await?;

        Ok(featured_theme.map(|f| f.into()))
    }
}
