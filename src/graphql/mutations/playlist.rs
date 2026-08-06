use animethemes_graphql_rust::entities::list::playlist;
use async_graphql::{Context, InputObject, Object, Result};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect,
};

use crate::graphql::{
    enums::list::playlistvisibility::PlaylistVisibility, types::list::playlist::Playlist,
};

#[derive(InputObject)]
struct CreatePlaylistInput {
    name: String,
    description: Option<String>,
    visibility: PlaylistVisibility,
}

#[derive(InputObject)]
struct UpdatePlaylistInput {
    name: Option<String>,
    description: Option<String>,
    visibility: Option<PlaylistVisibility>,
}

#[derive(Default)]
pub struct PlaylistMutation;

#[Object]
impl PlaylistMutation {
    async fn create_playlist(
        &self,
        ctx: &Context<'_>,
        input: CreatePlaylistInput,
    ) -> Result<Playlist> {
        let db = ctx.data::<DatabaseConnection>()?;

        let playlist = playlist::ActiveModel {
            name: Set(input.name),
            description: Set(input.description),
            visibility: Set(input.visibility.into()),
            ..Default::default()
        };

        let playlist = playlist.insert(db).await?;

        Ok(playlist.into())
    }

    async fn update_playlist(
        &self,
        ctx: &Context<'_>,
        id: String,
        input: UpdatePlaylistInput,
    ) -> Result<Playlist> {
        let db = ctx.data::<DatabaseConnection>()?;

        let model_id = playlist::Entity::find()
            .select_only()
            .column(playlist::Column::Id)
            .filter(playlist::Column::Hashid.eq(id))
            .into_tuple::<u64>()
            .one(db)
            .await?
            .unwrap();

        let mut playlist = playlist::ActiveModel {
            id: Set(model_id),
            ..Default::default()
        };

        if let Some(name) = input.name {
            playlist.name = Set(name);
        }

        if let Some(description) = input.description {
            playlist.description = Set(Some(description));
        }

        if let Some(visibility) = input.visibility {
            playlist.visibility = Set(visibility.into());
        }

        let playlist = playlist.update(db).await?;

        Ok(playlist.into())
    }

    async fn delete_playlist(&self, ctx: &Context<'_>, id: String) -> Result<bool> {
        let db = ctx.data::<DatabaseConnection>()?;

        let result = playlist::Entity::delete_many()
            .filter(playlist::Column::Hashid.eq(id))
            .exec(db)
            .await?;

        Ok(result.rows_affected > 0)
    }
}
