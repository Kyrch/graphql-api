use async_graphql::Enum;

use crate::enums::content::animeformat::AnimeFormat as AnimeFormatEnum;

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
