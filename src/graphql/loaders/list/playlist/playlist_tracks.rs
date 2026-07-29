use std::collections::HashMap;

use async_graphql::dataloader::Loader;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::entities::list::track;

pub struct PlaylistTracksLoader {
    pub db: DatabaseConnection,
}

impl Loader<u64> for PlaylistTracksLoader {
    type Value = Vec<track::Model>;
    type Error = sea_orm::DbErr;

    async fn load(&self, keys: &[u64]) -> Result<HashMap<u64, Self::Value>, Self::Error> {
        let tracks = track::Entity::find()
            .filter(track::Column::PlaylistId.is_in(keys))
            .all(&self.db)
            .await?;

        let mut result: HashMap<u64, Vec<track::Model>> = HashMap::new();

        for track in tracks {
            result.entry(track.playlist_id).or_default().push(track);
        }

        Ok(result)
    }
}
