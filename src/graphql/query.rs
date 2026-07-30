use async_graphql::{
    Context, InputObject, Object, Result,
    connection::{Connection, EmptyFields},
};

use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
    entities::{
        admin::featuredtheme,
        auth::user,
        content::{anime, artist, series, studio},
        document::page,
        list::playlist,
    },
    graphql::{
        inputs::pagination_input::PaginationInput,
        types::{
            admin::featuredtheme::FeaturedTheme,
            auth::me::Me,
            content::{anime::Anime, artist::Artist, series::Series, studio::Studio},
            document::page::Page,
            list::playlist::Playlist,
        },
        utils::cursor_paginate,
    },
};

#[derive(InputObject, Default)]
struct AnimeFilterInput {
    title: Option<String>,
}

pub struct Query;

#[Object]
impl Query {
    async fn current_featured_theme(&self, ctx: &Context<'_>) -> Result<Option<FeaturedTheme>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let featured_theme = featuredtheme::Entity::find()
            .filter(featuredtheme::Column::StartAt.lte(chrono::Utc::now()))
            .filter(featuredtheme::Column::EndAt.gte(chrono::Utc::now()))
            .one(db)
            .await?;

        Ok(featured_theme.map(|f| f.into()))
    }

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

    async fn anime_connection(
        &self,
        ctx: &Context<'_>,
        pagination: Option<PaginationInput>,
        filter: Option<AnimeFilterInput>,
    ) -> Result<Connection<u64, Anime, EmptyFields, EmptyFields>> {
        let mut query = anime::Entity::find();

        let filter = filter.unwrap_or_default();

        if let Some(title) = filter.title {
            query = query.filter(anime::Column::Title.like(title))
        }

        cursor_paginate(
            query,
            ctx,
            anime::Column::Id,
            pagination,
            |model: &anime::Model| model.id,
        )
        .await
    }
}
