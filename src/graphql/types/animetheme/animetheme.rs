use async_graphql::{Enum, SimpleObject};

use crate::entities::animetheme::animetheme::{self, ThemeType as ThemeTypeEnum};

#[derive(SimpleObject)]
pub struct AnimeTheme {
    pub sequence: Option<i32>,
    pub slug: String,
    #[graphql(name = "type")]
    pub themetype: ThemeType,
}

impl From<animetheme::Model> for AnimeTheme {
    fn from(model: animetheme::Model) -> Self {
        Self {
            sequence: model.sequence,
            slug: model.slug,
            themetype: model.themetype.into(),
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum ThemeType {
    OP,
    ED,
    IN,
}

impl From<ThemeTypeEnum> for ThemeType {
    fn from(value: ThemeTypeEnum) -> Self {
        match value {
            ThemeTypeEnum::OP => ThemeType::OP,
            ThemeTypeEnum::ED => ThemeType::ED,
            ThemeTypeEnum::IN => ThemeType::IN,
        }
    }
}
