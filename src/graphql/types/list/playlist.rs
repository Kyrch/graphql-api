use animethemes_graphql_rust::enums::LocalizedEnum;
use async_graphql::{ComplexObject, Context, Result, SimpleObject, dataloader::DataLoader};

use crate::{
    entities::list::playlist::{self},
    graphql::{
        enums::list::playlistvisibility::PlaylistVisibility,
        loaders::list::playlist::{
            playlist_track_first_last::PlaylistTrackFirstLastLoader,
            playlist_tracks::PlaylistTracksLoader, playlist_user::PlaylistUserLoader,
        },
        types::{auth::user::User, list::track::PlaylistTrack},
    },
};

/// Represents a list of ordered tracks intended for continuous playback.
///
/// For example, a "/r/anime's Best OPs and EDs of 2022" playlist may contain a collection of tracks allowing the continuous playback of Best OP and ED nominations for the /r/anime Awards.
#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Playlist {
    /// The primary key of the resource
    pub id: u64,
    #[graphql(skip)]
    pub user_id: Option<u64>,
    #[graphql(skip)]
    pub first_id: Option<u64>,
    #[graphql(skip)]
    pub last_id: Option<u64>,
    /// The title of the playlist
    pub name: String,
    /// The description of the playlist
    pub description: Option<String>,
    /// The state of who can see the playlist
    pub visibility: PlaylistVisibility,
    /// The localized string value of the visibility field
    pub visibility_localized: String,
}

#[ComplexObject]
impl Playlist {
    async fn tracks_count(&self, _ctx: &Context<'_>) -> Result<i32> {
        todo!()
    }

    async fn tracks_exists(&self, _ctx: &Context<'_>) -> Result<bool> {
        todo!()
    }

    async fn user(&self, ctx: &Context<'_>) -> Result<Option<User>> {
        let Some(user_id) = self.user_id else {
            return Ok(None);
        };

        let loader = ctx.data::<DataLoader<PlaylistUserLoader>>()?;

        Ok(loader.load_one(user_id).await?.map(Into::into))
    }

    async fn first(&self, ctx: &Context<'_>) -> Result<Option<PlaylistTrack>> {
        let Some(first_id) = self.first_id else {
            return Ok(None);
        };

        let loader = ctx.data::<DataLoader<PlaylistTrackFirstLastLoader>>()?;

        Ok(loader.load_one(first_id).await?.map(Into::into))
    }

    async fn last(&self, ctx: &Context<'_>) -> Result<Option<PlaylistTrack>> {
        let Some(last_id) = self.last_id else {
            return Ok(None);
        };

        let loader = ctx.data::<DataLoader<PlaylistTrackFirstLastLoader>>()?;

        Ok(loader.load_one(last_id).await?.map(Into::into))
    }

    async fn tracks(&self, ctx: &Context<'_>) -> Result<Vec<PlaylistTrack>> {
        let loader = ctx.data::<DataLoader<PlaylistTracksLoader>>()?;

        let models = loader.load_one(self.id).await?.unwrap_or_default();

        Ok(models.into_iter().map(PlaylistTrack::from).collect())
    }
}

impl From<playlist::Model> for Playlist {
    fn from(model: playlist::Model) -> Self {
        Self {
            id: model.id,
            user_id: model.user_id,
            first_id: model.first_id,
            last_id: model.last_id,
            name: model.name,
            description: model.description,
            visibility: model.visibility.into(),
            visibility_localized: model.visibility.localize().to_string(),
        }
    }
}
