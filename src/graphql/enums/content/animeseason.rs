use async_graphql::Enum;

use crate::enums::content::animeseason::AnimeSeason as AnimeSeasonEnum;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum AnimeSeason {
    Winter,
    Spring,
    Summer,
    Fall,
}

impl From<AnimeSeasonEnum> for AnimeSeason {
    fn from(value: AnimeSeasonEnum) -> Self {
        match value {
            AnimeSeasonEnum::Winter => AnimeSeason::Winter,
            AnimeSeasonEnum::Spring => AnimeSeason::Spring,
            AnimeSeasonEnum::Summer => AnimeSeason::Summer,
            AnimeSeasonEnum::Fall => AnimeSeason::Fall,
        }
    }
}
