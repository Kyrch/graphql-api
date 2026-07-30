use async_graphql::{Context, MergedObject, Object, Result};

use sea_orm::{DatabaseConnection, EntityTrait};

use crate::{
    entities::auth::user,
    graphql::{
        queries::{
            admin::{announcement::AnnouncementQuery, featuredtheme::FeaturedThemeQuery},
            content::{
                anime::AnimeQuery, artist::ArtistQuery, series::SeriesQuery, studio::StudioQuery,
            },
            document::page::PageQuery,
            list::playlist::PlaylistQuery,
        },
        types::auth::me::Me,
    },
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

#[Object]
impl RootQuery {
    async fn me(&self, ctx: &Context<'_>) -> Result<Option<Me>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let user = user::Entity::find_by_id(1u64).one(db).await?;

        Ok(user.map(|a| a.into()))
    }
}
