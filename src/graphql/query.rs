use async_graphql::{Context, MergedObject, Object, Result};

use crate::graphql::{
    queries::{
        admin::{announcement::AnnouncementQuery, featuredtheme::FeaturedThemeQuery},
        content::{
            anime::AnimeQuery, animetheme::AnimeThemeQuery, animeyear::AnimeYearQuery,
            artist::ArtistQuery, image::ImageQuery, series::SeriesQuery, studio::StudioQuery,
        },
        document::page::PageQuery,
        list::playlist::PlaylistQuery,
        search::SearchQuery,
    },
    types::auth::me::Me,
};

#[derive(MergedObject, Default)]
pub struct Query(
    RootQuery,
    SearchQuery,
    AnnouncementQuery,
    FeaturedThemeQuery,
    PageQuery,
    PlaylistQuery,
    AnimeQuery,
    AnimeYearQuery,
    AnimeThemeQuery,
    ArtistQuery,
    ImageQuery,
    SeriesQuery,
    StudioQuery,
);

#[derive(Default)]
struct RootQuery;

#[Object]
impl RootQuery {
    async fn me(&self, _ctx: &Context<'_>) -> Result<Option<Me>> {
        todo!()
    }
}
