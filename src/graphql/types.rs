use async_graphql::{Enum, SimpleObject};

use crate::entities::anime::{
    self,
    AnimeFormat as AnimeFormatEnum,
    AnimeSeason as AnimeSeasonEnum
};

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum AnimeFormat {
    TV,
    TVShort,
    OVA,
    Movie,
    Special,
    ONA,
}

impl From<AnimeFormatEnum> for AnimeFormat {
    fn from(value: AnimeFormatEnum) -> Self {
        match value {
            AnimeFormatEnum::TV => AnimeFormat::TV,
            AnimeFormatEnum::TVShort => AnimeFormat::TVShort,
            AnimeFormatEnum::OVA => AnimeFormat::OVA,
            AnimeFormatEnum::Movie => AnimeFormat::Movie,
            AnimeFormatEnum::Special => AnimeFormat::Special,
            AnimeFormatEnum::ONA => AnimeFormat::ONA,
        }
    }
}

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

#[derive(SimpleObject)]
pub struct AnimeTitle {
    romaji: String,
    english: Option<String>,
    native: Option<String>,
}

impl From<&anime::Model> for AnimeTitle {
    fn from(model: &anime::Model) -> Self {
        Self {
            romaji: model.title.clone(),
            english: model.title_english.clone(),
            native: model.title_native.clone(),
        }
    }
}

#[derive(SimpleObject)]
pub struct Anime {
    pub id: u64,
    pub format: AnimeFormat,
    pub season: AnimeSeason,
    pub slug: String,
    pub synopsis: Option<String>,
    pub title: AnimeTitle,
    pub year: i32,
}