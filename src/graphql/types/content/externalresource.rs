use async_graphql::{Enum, SimpleObject};

use crate::entities::content::externalresource::{self, ResourceSite as ResourceSiteEnum};

/// Represents a site with supplementary information for another resource such as an anime or artist.
///
/// For example, the Bakemonogatari anime has MyAnimeList, AniList and AniDB resources.
#[derive(SimpleObject)]
pub struct ExternalResource {
    /// The primary key of the resource
    pub id: u64,
    /// The primary key of the resource in the external site
    pub external_id: Option<i32>,
    /// The URL of the external site
    pub link: String,
    /// The external site that the resource belongs to
    pub site: ResourceSite,
    /// The localized external site that the resource belongs to
    pub site_localized: String,
}

impl From<externalresource::Model> for ExternalResource {
    fn from(model: externalresource::Model) -> Self {
        Self {
            id: model.id,
            external_id: model.external_id,
            link: model.link,
            site: model.site.into(),
            site_localized: format!("{:?}", model.site),
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum ResourceSite {
    OfficialSite,
    X,
    Anidb,
    Anilist,
    AnimePlanet,
    ANN,
    Kitsu,
    MAL,
    Wiki,
    Spotify,
    YoutubeMusic,
    Youtube,
    AppleMusic,
    AmazonMusic,
    Crunchyroll,
    Hidive,
    Netflix,
    DisneyPlus,
    Hulu,
    AmazonPrimeVideo,
    Livechart,
}

impl From<ResourceSiteEnum> for ResourceSite {
    fn from(value: ResourceSiteEnum) -> Self {
        match value {
            ResourceSiteEnum::OfficialSite => ResourceSite::OfficialSite,
            ResourceSiteEnum::X => ResourceSite::X,
            ResourceSiteEnum::Anidb => ResourceSite::Anidb,
            ResourceSiteEnum::Anilist => ResourceSite::Anilist,
            ResourceSiteEnum::AnimePlanet => ResourceSite::AnimePlanet,
            ResourceSiteEnum::ANN => ResourceSite::ANN,
            ResourceSiteEnum::Kitsu => ResourceSite::Kitsu,
            ResourceSiteEnum::MAL => ResourceSite::MAL,
            ResourceSiteEnum::Wiki => ResourceSite::Wiki,
            ResourceSiteEnum::Spotify => ResourceSite::Spotify,
            ResourceSiteEnum::YoutubeMusic => ResourceSite::YoutubeMusic,
            ResourceSiteEnum::Youtube => ResourceSite::Youtube,
            ResourceSiteEnum::AppleMusic => ResourceSite::AppleMusic,
            ResourceSiteEnum::AmazonMusic => ResourceSite::AmazonMusic,
            ResourceSiteEnum::Crunchyroll => ResourceSite::Crunchyroll,
            ResourceSiteEnum::Hidive => ResourceSite::Hidive,
            ResourceSiteEnum::Netflix => ResourceSite::Netflix,
            ResourceSiteEnum::DisneyPlus => ResourceSite::DisneyPlus,
            ResourceSiteEnum::Hulu => ResourceSite::Hulu,
            ResourceSiteEnum::AmazonPrimeVideo => ResourceSite::AmazonPrimeVideo,
            ResourceSiteEnum::Livechart => ResourceSite::Livechart,
        }
    }
}
