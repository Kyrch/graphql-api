use async_graphql::{Context, Object, Result};

use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
    entities::{
        auth::user,
        content::{anime, artist, series, studio},
        document::page,
        list::playlist,
    },
    graphql::types::{
        auth::me::Me,
        content::{anime::Anime, artist::Artist, series::Series, studio::Studio},
        document::page::Page,
        list::playlist::Playlist,
    },
};

pub struct Query;

#[Object]
impl Query {
    async fn me(&self, ctx: &Context<'_>) -> Result<Option<Me>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let user = user::Entity::find_by_id(1u64).one(db).await?;

        Ok(user.map(|a| a.into()))
    }

    async fn playlist(&self, ctx: &Context<'_>, id: String) -> Result<Option<Playlist>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let playlist = playlist::Entity::find()
            .filter(playlist::Column::Hashid.eq(id))
            .one(db)
            .await?;

        Ok(playlist.map(|a| a.into()))
    }

    async fn page(&self, ctx: &Context<'_>, slug: String) -> Result<Option<Page>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let page = page::Entity::find()
            .filter(page::Column::Slug.eq(slug))
            .one(db)
            .await?;

        Ok(page.map(|a| a.into()))
    }

    async fn anime(&self, ctx: &Context<'_>, slug: String) -> Result<Option<Anime>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let anime = anime::Entity::find()
            .filter(anime::Column::Slug.eq(slug))
            .one(db)
            .await?;

        Ok(anime.map(|a| a.into()))
    }

    async fn artist(&self, ctx: &Context<'_>, slug: String) -> Result<Option<Artist>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let artist = artist::Entity::find()
            .filter(artist::Column::Slug.eq(slug))
            .one(db)
            .await?;

        Ok(artist.map(|a| a.into()))
    }

    async fn series(&self, ctx: &Context<'_>, slug: String) -> Result<Option<Series>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let series = series::Entity::find()
            .filter(series::Column::Slug.eq(slug))
            .one(db)
            .await?;

        Ok(series.map(|a| a.into()))
    }

    async fn studio(&self, ctx: &Context<'_>, slug: String) -> Result<Option<Studio>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let studio: Option<studio::Model> = studio::Entity::find()
            .filter(studio::Column::Slug.eq(slug))
            .one(db)
            .await?;

        Ok(studio.map(|a| a.into()))
    }

    async fn anime_all(&self, ctx: &Context<'_>) -> Result<Vec<Anime>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let anime = anime::Entity::find().all(db).await?;

        Ok(anime.into_iter().map(Into::into).collect())
    }
}
