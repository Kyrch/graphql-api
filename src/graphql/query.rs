use async_graphql::{Context, MergedObject, Object, Result, SimpleObject};

use axum::Error;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use typesense::models::SearchParameters;

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
    typesense::{client::TypesenseClient, documents::anime_document::AnimeDocument},
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
            search_struct.anime = self.search_anime(ctx, search.clone()).await?;
        }

        Ok(search_struct)
    }

    async fn search_anime(&self, ctx: &Context<'_>, search: String) -> Result<Vec<Anime>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let typesense = ctx.data::<TypesenseClient>()?;

        let anime_document = typesense
            .collection::<AnimeDocument>()
            .documents()
            .search(
                SearchParameters::builder()
                    .q(search)
                    .query_by("title,title_english,title_native,synonyms")
                    .query_by_weights("8,6,6,5")
                    .build(),
            )
            .await
            .map_err(|error| Error::new(error.to_string()))?;

        let ids: Vec<String> = anime_document
            .hits
            .unwrap_or_default()
            .into_iter()
            .filter_map(|hit| hit.document)
            .filter_map(|document| document.id.parse::<String>().ok())
            .collect();

        let animes = anime::Entity::find()
            .filter(anime::Column::Id.is_in(ids))
            .all(db)
            .await?;

        Ok(animes.into_iter().map(|a| a.into()).collect())
    }
}
