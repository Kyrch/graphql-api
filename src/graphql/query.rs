use animethemes_graphql_rust::{
    entities::content::{animetheme::animetheme, song, studio},
    typesense::{
        documents::{
            anime_document::{self, AnimeDocument},
            animetheme_document::{self, AnimeThemeDocument},
            song_document::{self, SongDocument},
            studio_document::{self, StudioDocument},
        },
        search::search as search_function,
    },
};
use async_graphql::{Context, MergedObject, Object, Result, SimpleObject};

use sea_orm::{DatabaseConnection, EntityTrait};

use crate::{
    entities::{auth::user, content::anime},
    graphql::{
        queries::{
            admin::{announcement::AnnouncementQuery, featuredtheme::FeaturedThemeQuery},
            content::{
                anime::AnimeQuery, artist::ArtistQuery, series::SeriesQuery, studio::StudioQuery,
            },
            document::page::PageQuery,
            list::playlist::PlaylistQuery,
        },
        types::{
            auth::me::Me,
            content::{
                anime::Anime, animetheme::animetheme::AnimeTheme, artist::Artist, series::Series,
                song::Song, studio::Studio, video::Video,
            },
            list::playlist::Playlist,
        },
    },
    typesense::client::TypesenseClient,
};

#[derive(MergedObject, Default)]
pub struct Query(
    RootQuery,
    AnnouncementQuery,
    FeaturedThemeQuery,
    PageQuery,
    PlaylistQuery,
    AnimeQuery,
    ArtistQuery,
    SeriesQuery,
    StudioQuery,
);

#[derive(Default)]
struct RootQuery;

/// Returns a listing of resources that match a given search term.
#[derive(SimpleObject)]
struct Search {
    /// The anime results of the search
    anime: Vec<Anime>,
    /// The artist results of the search
    artists: Vec<Artist>,
    /// The theme results of the search
    animethemes: Vec<AnimeTheme>,
    /// The playlist results of the search
    playlists: Vec<Playlist>,
    /// The series results of the search
    series: Vec<Series>,
    /// The song results of the search
    songs: Vec<Song>,
    /// The studio results of the search
    studios: Vec<Studio>,
    /// The video results of the search
    videos: Vec<Video>,
}

#[Object]
impl RootQuery {
    async fn me(&self, ctx: &Context<'_>) -> Result<Option<Me>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let user = user::Entity::find_by_id(1u64).one(db).await?;

        Ok(user.map(|a| a.into()))
    }

    async fn search(&self, ctx: &Context<'_>, search: String) -> Result<Search> {
        let db = ctx.data::<DatabaseConnection>()?;

        let typesense = ctx.data::<TypesenseClient>()?;

        let mut search_struct = Search {
            anime: Vec::new(),
            artists: Vec::new(),
            animethemes: Vec::new(),
            playlists: Vec::new(),
            series: Vec::new(),
            songs: Vec::new(),
            studios: Vec::new(),
            videos: Vec::new(),
        };

        if ctx.look_ahead().field("anime").exists() {
            search_struct.anime = search_function::<anime::Entity, AnimeDocument>(
                db,
                typesense,
                anime::Entity::find(),
                anime::Column::Id,
                search.clone(),
                anime_document::QUERY_BY,
                anime_document::QUERY_BY_WEIGHTS,
            )
            .await?
            .into_iter()
            .map(|m| m.into())
            .collect();
        }

        if ctx.look_ahead().field("animethemes").exists() {
            search_struct.animethemes = search_function::<animetheme::Entity, AnimeThemeDocument>(
                db,
                typesense,
                animetheme::Entity::find(),
                animetheme::Column::Id,
                search.clone(),
                animetheme_document::QUERY_BY,
                animetheme_document::QUERY_BY_WEIGHTS,
            )
            .await?
            .into_iter()
            .map(|m| m.into())
            .collect();
        }

        if ctx.look_ahead().field("songs").exists() {
            search_struct.songs = search_function::<song::Entity, SongDocument>(
                db,
                typesense,
                song::Entity::find(),
                song::Column::Id,
                search.clone(),
                song_document::QUERY_BY,
                song_document::QUERY_BY_WEIGHTS,
            )
            .await?
            .into_iter()
            .map(|m| m.into())
            .collect();
        }

        if ctx.look_ahead().field("studios").exists() {
            search_struct.studios = search_function::<studio::Entity, StudioDocument>(
                db,
                typesense,
                studio::Entity::find(),
                studio::Column::Id,
                search.clone(),
                studio_document::QUERY_BY,
                studio_document::QUERY_BY_WEIGHTS,
            )
            .await?
            .into_iter()
            .map(|m| m.into())
            .collect();
        }

        Ok(search_struct)
    }
}
