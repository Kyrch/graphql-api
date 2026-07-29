use async_graphql::Enum;

use crate::enums::content::themetype::ThemeType as ThemeTypeEnum;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum ThemeType {
    /// Opening
    OP,
    /// Ending
    ED,
    /// Insert Song
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
