use async_graphql::Enum;

use crate::enums::content::imagefacet::ImageFacet as ImageFacetEnum;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum ImageFacet {
    SmallCover,
    LargeCover,
    Grill,
    Document,
    Avatar,
    Banner,
}

impl From<ImageFacetEnum> for ImageFacet {
    fn from(value: ImageFacetEnum) -> Self {
        match value {
            ImageFacetEnum::SmallCover => ImageFacet::SmallCover,
            ImageFacetEnum::LargeCover => ImageFacet::LargeCover,
            ImageFacetEnum::Grill => ImageFacet::Grill,
            ImageFacetEnum::Document => ImageFacet::Document,
            ImageFacetEnum::Avatar => ImageFacet::Avatar,
            ImageFacetEnum::Banner => ImageFacet::Banner,
        }
    }
}
