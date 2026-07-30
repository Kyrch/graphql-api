use async_graphql::{
    Context, InputObject, Object, Result,
    connection::{Connection, EmptyFields},
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
    entities::list::playlist,
    graphql::{
        inputs::pagination_input::PaginationInput, types::list::playlist::Playlist,
        utils::cursor_paginate,
    },
    scopes::list::playlist::public_playlists,
};

#[derive(InputObject, Default)]
struct PlaylistFilterInput {
    name_like: Option<String>,
}

#[derive(Default)]
pub struct PlaylistQuery;

#[Object]
impl PlaylistQuery {
    async fn playlist(&self, ctx: &Context<'_>, id: String) -> Result<Option<Playlist>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let playlist = playlist::Entity::find()
            .filter(playlist::Column::Hashid.eq(id))
            .one(db)
            .await?;

        Ok(playlist.map(|a| a.into()))
    }

    async fn playlist_connection(
        &self,
        ctx: &Context<'_>,
        pagination: Option<PaginationInput>,
        filter: Option<PlaylistFilterInput>,
    ) -> Result<Connection<u64, Playlist, EmptyFields, EmptyFields>> {
        let mut query: sea_orm::prelude::Select<playlist::Entity> =
            playlist::Entity::find().filter(public_playlists());

        let filter = filter.unwrap_or_default();

        if let Some(name_like) = filter.name_like {
            query = query.filter(playlist::Column::Name.like(name_like))
        }

        cursor_paginate(
            query,
            ctx,
            playlist::Column::Id,
            pagination,
            |model: &playlist::Model| model.id,
        )
        .await
    }
}
