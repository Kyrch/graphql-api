use animethemes_graphql_rust::{
    entities::{
        content::{animetheme::animetheme, artist, series, song, studio, video},
        list::playlist,
    },
    typesense::search::{
        search_anime, search_animethemes, search_artists, search_playlists, search_series,
        search_songs, search_studios, search_videos,
    },
};
use async_graphql::{Context, MergedObject, Object, ObjectType, Result, SimpleObject};

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
            search_struct.anime = convert_type(
                search_anime(db, typesense, anime::Entity::find(), search.clone()).await?,
            );
        }

        if ctx.look_ahead().field("artists").exists() {
            search_struct.artists = convert_type(
                search_artists(db, typesense, artist::Entity::find(), search.clone()).await?,
            );
        }

        if ctx.look_ahead().field("animethemes").exists() {
            search_struct.animethemes = convert_type(
                search_animethemes(db, typesense, animetheme::Entity::find(), search.clone())
                    .await?,
            );
        }

        if ctx.look_ahead().field("playlists").exists() {
            search_struct.playlists = convert_type(
                search_playlists(db, typesense, playlist::Entity::find(), search.clone()).await?,
            );
        }

        if ctx.look_ahead().field("series").exists() {
            search_struct.series = convert_type(
                search_series(db, typesense, series::Entity::find(), search.clone()).await?,
            );
        }

        if ctx.look_ahead().field("songs").exists() {
            search_struct.songs = convert_type(
                search_songs(db, typesense, song::Entity::find(), search.clone()).await?,
            );
        }

        if ctx.look_ahead().field("studios").exists() {
            search_struct.studios = convert_type(
                search_studios(db, typesense, studio::Entity::find(), search.clone()).await?,
            );
        }

        if ctx.look_ahead().field("videos").exists() {
            search_struct.videos = convert_type(
                search_videos(db, typesense, video::Entity::find(), search.clone()).await?,
            );
        }

        Ok(search_struct)
    }
}

fn convert_type<T, M>(models: Vec<M>) -> Vec<T>
where
    T: ObjectType + From<M>,
{
    models.into_iter().map(T::from).collect()
}
