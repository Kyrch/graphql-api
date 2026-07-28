use async_graphql::{ComplexObject, Context, Result, SimpleObject, dataloader::DataLoader};

use crate::{
    entities::content::artist,
    graphql::{
        loaders::artist::artist_performances::ArtistPerformancesLoader,
        types::content::performance::Performance,
    },
};

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
#[graphql(complex)]
pub struct Artist {
    #[graphql(skip)]
    pub id: u64,
    /// The primary title of the artist
    pub name: ArtistName,
    /// The URL slug & route key of the resource
    pub slug: String,
    /// The brief information of the resource
    pub information: Option<String>,
}

#[ComplexObject]
impl Artist {
    async fn performances(&self, ctx: &Context<'_>) -> Result<Vec<Performance>> {
        let loader = ctx.data::<DataLoader<ArtistPerformancesLoader>>()?;

        let models = loader.load_one(self.id).await?.unwrap_or_default();

        Ok(models.into_iter().map(Performance::from).collect())
    }
}

impl From<artist::Model> for Artist {
    fn from(model: artist::Model) -> Self {
        let name = ArtistName::from(&model);
        Self {
            id: model.id,
            slug: model.slug,
            name,
            information: model.information,
        }
    }
}
