use async_graphql::{Enum, SimpleObject};

use crate::entities::anime;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum AnimeFormat {
    TV,
    TVShort,
    OVA,
    Movie,
    Special,
    ONA,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum AnimeSeason {
    Winter,
    Spring,
    Summer,
    Fall,
}

impl From<i32> for AnimeFormat {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::TV,
            1 => Self::TVShort,
            2 => Self::OVA,
            3 => Self::Movie,
            4 => Self::Special,
            5 => Self::ONA,
            _ => panic!("Invalid AnimeFormat: {}", value),
        }
    }
}

impl From<i32> for AnimeSeason {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::Winter,
            1 => Self::Spring,
            2 => Self::Summer,
            3 => Self::Fall,
            _ => panic!("Invalid AnimeSeason: {}", value),
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
    pub title: AnimeTitle,
    pub year: i32,
}