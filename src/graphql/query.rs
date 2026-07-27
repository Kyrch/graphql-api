use async_graphql::{Context, Object, Result};

use sea_orm::{DatabaseConnection, EntityTrait};

use crate::{entities::anime, graphql::types::anime::Anime};

pub struct Query;

#[Object]
impl Query {
    async fn anime(&self, ctx: &Context<'_>, id: u64) -> Result<Option<Anime>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let anime = anime::Entity::find_by_id(id).one(db).await?;

        Ok(anime.map(|a| a.into()))
    }

    async fn anime_all(&self, ctx: &Context<'_>) -> Result<Vec<Anime>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let anime = anime::Entity::find().all(db).await?;

        Ok(anime.into_iter().map(Into::into).collect())
    }
}
