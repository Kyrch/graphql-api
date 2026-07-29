use async_graphql::{Enum, SimpleObject};

use crate::entities::content::image::{self, ImageFacet as ImageFacetEnum};

/// Represents a visual component for another resource such as an anime or artist.
///
/// For example, the Bakemonogatari anime has two images to represent small and large cover images.
#[derive(SimpleObject)]
pub struct Image {
    /// The primary key of the resource
    pub id: u64,
    /// The component that the resource is intended for
    pub facet: ImageFacet,
    /// The path of the file in storage
    pub path: String,
    /// The URL to stream the file from storage
    pub link: String,
}

impl From<image::Model> for Image {
    fn from(model: image::Model) -> Self {
        Self {
            id: model.id,
            facet: model.facet.into(),
            path: model.path,
            link: "".to_string(),
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
