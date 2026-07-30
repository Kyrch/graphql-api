use async_graphql::{
    Context, InputObject, Object, Result,
    connection::{Connection, EmptyFields},
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
    entities::content::anime,
    graphql::{
        inputs::pagination_input::PaginationInput, types::content::anime::Anime,
        utils::cursor_paginate,
    },
    scopes::{
        content::anime::{anime_by_title_like, anime_by_year},
        without_trashed,
    },
};

#[derive(InputObject, Default)]
struct AnimeFilterInput {
    title_like: Option<String>,
    year: Option<i32>,
}

#[derive(Default)]
pub struct AnimeQuery;

#[Object]
impl AnimeQuery {
    async fn anime(&self, ctx: &Context<'_>, slug: String) -> Result<Option<Anime>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let anime = anime::Entity::find()
            .filter(without_trashed::<anime::Entity>())
            .filter(anime::Column::Slug.eq(slug))
            .one(db)
            .await?;

        Ok(anime.map(|a| a.into()))
    }

    async fn anime_connection(
        &self,
        ctx: &Context<'_>,
        pagination: Option<PaginationInput>,
        filter: Option<AnimeFilterInput>,
    ) -> Result<Connection<u64, Anime, EmptyFields, EmptyFields>> {
        let mut query = anime::Entity::find().filter(without_trashed::<anime::Entity>());

        let filter = filter.unwrap_or_default();

        if let Some(title_like) = filter.title_like {
            query = query.filter(anime_by_title_like(title_like))
        }

        if let Some(year) = filter.year {
            query = query.filter(anime_by_year(year))
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
