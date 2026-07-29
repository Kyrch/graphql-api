use async_graphql::Enum;

use crate::enums::content::resourcesite::ResourceSite as ResourceSiteEnum;

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
