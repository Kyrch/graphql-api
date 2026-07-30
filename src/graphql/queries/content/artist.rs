use async_graphql::{Context, Object, Result};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
    entities::content::artist, graphql::types::content::artist::Artist, scopes::without_trashed,
};

#[derive(Default)]
pub struct ArtistQuery;

#[Object]
impl ArtistQuery {
    async fn artist(&self, ctx: &Context<'_>, slug: String) -> Result<Option<Artist>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let artist = artist::Entity::find()
            .filter(without_trashed::<artist::Entity>())
            .filter(artist::Column::Slug.eq(slug))
            .one(db)
            .await?;

        Ok(artist.map(|a| a.into()))
    }
}
