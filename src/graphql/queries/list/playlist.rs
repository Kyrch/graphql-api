use async_graphql::{Context, Object, Result};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{entities::list::playlist, graphql::types::list::playlist::Playlist};

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
}
