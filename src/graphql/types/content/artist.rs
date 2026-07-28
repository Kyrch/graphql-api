use async_graphql::SimpleObject;

use crate::entities::content::artist;

#[derive(SimpleObject)]
pub struct ArtistName {
    /// The stylized name of the artist
    main: String,
    /// The native name of the artist
    native: Option<String>,
}

impl From<&artist::Model> for ArtistName {
    fn from(model: &artist::Model) -> Self {
        Self {
            main: model.name.clone(),
            native: model.name_native.clone(),
        }
    }
}

/// Represents a musical performer of anime sequences.
///
/// For example, Chiwa Saitou is the musical performer of the Bakemonogatari OP1 theme, among many others.
#[derive(SimpleObject)]
pub struct Artist {
    /// The primary title of the artist
    pub name: ArtistName,
    /// The URL slug & route key of the resource
    pub slug: String,
    /// The brief information of the resource
    pub information: Option<String>,
}

impl From<artist::Model> for Artist {
    fn from(model: artist::Model) -> Self {
        let name = ArtistName::from(&model);
        Self {
            slug: model.slug,
            name,
            information: model.information,
        }
    }
}
