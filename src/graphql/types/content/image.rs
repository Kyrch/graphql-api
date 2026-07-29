use async_graphql::{Enum, SimpleObject};

use crate::entities::content::image::{self, ImageFacet as ImageFacetEnum};

/// Represents a visual component for another resource such as an anime or artist.
///
/// For example, the Bakemonogatari anime has two images to represent small and large cover images.
#[derive(SimpleObject)]
pub struct Image {
    pub facet: ImageFacet,
    pub path: String,
}

impl From<image::Model> for Image {
    fn from(model: image::Model) -> Self {
        Self {
            facet: model.facet.into(),
            path: model.path,
        }
    }
}

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
