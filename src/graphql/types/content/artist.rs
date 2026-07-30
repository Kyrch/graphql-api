use async_graphql::{
    ComplexObject, Context, Result, SimpleObject,
    connection::{Connection, Edge, EmptyFields},
    dataloader::DataLoader,
};

use crate::{
    entities::content::artist,
    graphql::{
        loaders::content::{
            artist::artist_performances::ArtistPerformancesLoader,
            imageable::{ImageableKey, ImageableLoader},
            resourceable::{ResourceableKey, ResourceableLoader},
        },
        types::content::{
            externalresource::ExternalResource,
            image::Image,
            imageable::{ImageEdgeFields, ImageableConnection, ImageableEdge},
            performance::Performance,
            resourceable::{ExternalResourceEdgeFields, ResourceableConnection, ResourceableEdge},
        },
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
    /// The primary key of the resource
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

    async fn images(
        &self,
        ctx: &Context<'_>,
    ) -> Result<
        Connection<u64, Image, EmptyFields, ImageEdgeFields, ImageableConnection, ImageableEdge>,
    > {
        let loader = ctx.data::<DataLoader<ImageableLoader>>()?;

        let rows = loader
            .load_one(ImageableKey {
                id: self.id,
                imageable_type: "artist".to_string(),
            })
            .await?
            .unwrap_or_default();

        let mut connection = Connection::with_additional_fields(false, false, EmptyFields);

        for (pivot, image) in rows {
            connection.edges.push(Edge::with_additional_fields(
                image.id,
                image.into(),
                ImageEdgeFields {
                    depth: pivot.depth,
                    created_at: pivot.created_at,
                    updated_at: pivot.updated_at,
                },
            ));
        }

        Ok(connection)
    }

    async fn resources(
        &self,
        ctx: &Context<'_>,
    ) -> Result<
        Connection<
            u64,
            ExternalResource,
            EmptyFields,
            ExternalResourceEdgeFields,
            ResourceableConnection,
            ResourceableEdge,
        >,
    > {
        let loader = ctx.data::<DataLoader<ResourceableLoader>>()?;

        let rows = loader
            .load_one(ResourceableKey {
                id: self.id,
                resourceable_type: "artist".to_string(),
            })
            .await?
            .unwrap_or_default();

        let mut connection = Connection::with_additional_fields(false, false, EmptyFields);

        for (pivot, resource) in rows {
            connection.edges.push(Edge::with_additional_fields(
                resource.id,
                resource.into(),
                ExternalResourceEdgeFields {
                    r#as: pivot.r#as,
                    created_at: pivot.created_at,
                    updated_at: pivot.updated_at,
                },
            ));
        }

        Ok(connection)
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
