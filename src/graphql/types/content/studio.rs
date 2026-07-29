use async_graphql::{
    ComplexObject, Context, Result, SimpleObject,
    connection::{Connection, Edge, EmptyFields},
    dataloader::DataLoader,
};

use crate::{
    entities::content::studio,
    graphql::{
        loaders::{
            imageable::{ImageableKey, ImageableLoader},
            resourceable::{ResourceableKey, ResourceableLoader},
        },
        types::content::{
            externalresource::ExternalResource, image::Image, imageable::ImageEdgeFields,
            resourceable::ExternalResourceEdgeFields,
        },
    },
};

/// Represents a company that produces anime.
///
/// For example, Shaft is the studio that produced the anime Bakemonogatari.
#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Studio {
    /// The primary key of the resource
    pub id: u64,
    /// The primary title of the Studio
    pub name: String,
    /// The URL slug & route key of the resource
    pub slug: String,
}

#[ComplexObject]
impl Studio {
    async fn images(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Connection<u64, Image, EmptyFields, ImageEdgeFields>> {
        let loader = ctx.data::<DataLoader<ImageableLoader>>()?;

        let rows = loader
            .load_one(ImageableKey {
                id: self.id,
                imageable_type: "studio".to_string(),
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
    ) -> Result<Connection<u64, ExternalResource, EmptyFields, ExternalResourceEdgeFields>> {
        let loader = ctx.data::<DataLoader<ResourceableLoader>>()?;

        let rows = loader
            .load_one(ResourceableKey {
                id: self.id,
                resourceable_type: "studio".to_string(),
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

impl From<studio::Model> for Studio {
    fn from(model: studio::Model) -> Self {
        Self {
            id: model.id,
            slug: model.slug,
            name: model.name,
        }
    }
}
