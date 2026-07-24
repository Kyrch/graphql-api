use async_graphql::{Context, Object, Result};

use sea_orm::{DatabaseConnection, EntityTrait};

use crate::entities::anime;
use crate::graphql::types::Anime;
use crate::graphql::types::AnimeTitle;

pub struct Query;


impl From<anime::Model> for Anime {
    fn from(model: anime::Model) -> Self {
        let title = AnimeTitle::from(&model);
        Self {
            id: model.id,
            format: model.format.into(),
            season: model.season.into(),
            slug: model.slug,
            title,
            year: model.year,
        }
    }
}

#[Object]
impl Query {
    async fn anime(
        &self,
        ctx: &Context<'_>,
        id: u64,
    ) -> Result<Option<Anime>> {

        let db = ctx.data::<DatabaseConnection>()?;

        let anime = anime::Entity::find_by_id(id)
            .one(db)
            .await?;

        Ok(anime.map(|a| a.into()))
    }

    async fn anime_all(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<Anime>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let anime = anime::Entity::find()
            .all(db)
            .await?;

        Ok(anime.into_iter().map(Into::into).collect())
    }
}