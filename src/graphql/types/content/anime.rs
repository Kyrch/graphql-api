use async_graphql::{
    ComplexObject, Context, Enum, Result, SimpleObject,
    connection::{Connection, Edge, EmptyFields},
    dataloader::DataLoader,
};

use crate::{
    entities::content::anime::{
        self, AnimeFormat as AnimeFormatEnum, AnimeSeason as AnimeSeasonEnum,
    },
    graphql::{
        loaders::content::{
            anime::{
                anime_series::AnimeSeriesLoader, anime_studios::AnimeStudiosLoader,
                anime_synonyms::AnimeSynonymsLoader, anime_themes::AnimeThemesLoader,
            },
            imageable::{ImageableKey, ImageableLoader},
            resourceable::{ResourceableKey, ResourceableLoader},
        },
        types::content::{
            anime_series::{AnimeSeriesConnection, AnimeSeriesEdge, AnimeSeriesEdgeFields},
            anime_studios::{AnimeStudioConnection, AnimeStudioEdge, AnimeStudioEdgeFields},
            animetheme::animetheme::AnimeTheme,
            externalresource::ExternalResource,
            image::Image,
            imageable::ImageEdgeFields,
            resourceable::ExternalResourceEdgeFields,
            series::Series,
            studio::Studio,
            synonym::Synonym,
        },
    },
};

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

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum AnimeSeason {
    Winter,
    Spring,
    Summer,
    Fall,
}

impl From<AnimeSeasonEnum> for AnimeSeason {
    fn from(value: AnimeSeasonEnum) -> Self {
        match value {
            AnimeSeasonEnum::Winter => AnimeSeason::Winter,
            AnimeSeasonEnum::Spring => AnimeSeason::Spring,
            AnimeSeasonEnum::Summer => AnimeSeason::Summer,
            AnimeSeasonEnum::Fall => AnimeSeason::Fall,
        }
    }
}

#[derive(SimpleObject)]
pub struct AnimeTitle {
    romaji: String,
    english: Option<String>,
    native: Option<String>,
}

impl From<&anime::Model> for AnimeTitle {
    fn from(model: &anime::Model) -> Self {
        Self {
            romaji: model.title.clone(),
            english: model.title_english.clone(),
            native: model.title_native.clone(),
        }
    }
}

/// Represents a production with at least one opening or ending sequence.
///
/// For example, Bakemonogatari is an anime production with five opening sequences and one ending sequence.
#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Anime {
    /// The primary key of the resource
    pub id: u64,
    pub name: String,
    /// The primary title of the anime
    pub title: AnimeTitle,
    /// The format of the anime
    pub format: Option<AnimeFormat>,
    /// The localized format of the anime
    pub format_localized: Option<String>,
    /// The premiere season of the anime
    pub season: Option<AnimeSeason>,
    /// The localized season of the anime
    pub season_localized: Option<String>,
    /// The URL slug & route key of the resource
    pub slug: String,
    /// The brief summary of the anime
    pub synopsis: Option<String>,
    /// The premiere season year of the anime
    pub year: Option<i32>,
}

#[ComplexObject]
impl Anime {
    async fn synonyms(&self, ctx: &Context<'_>) -> Result<Vec<Synonym>> {
        let loader = ctx.data::<DataLoader<AnimeSynonymsLoader>>()?;

        let models = loader.load_one(self.id).await?.unwrap_or_default();

        Ok(models.into_iter().map(Synonym::from).collect())
    }

    async fn animethemes(&self, ctx: &Context<'_>) -> Result<Vec<AnimeTheme>> {
        let loader = ctx.data::<DataLoader<AnimeThemesLoader>>()?;

        let models = loader.load_one(self.id).await?.unwrap_or_default();

        Ok(models.into_iter().map(AnimeTheme::from).collect())
    }

    async fn images(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Connection<u64, Image, EmptyFields, ImageEdgeFields>> {
        let loader = ctx.data::<DataLoader<ImageableLoader>>()?;

        let rows = loader
            .load_one(ImageableKey {
                id: self.id,
                imageable_type: "anime".to_string(),
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
                resourceable_type: "anime".to_string(),
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

    async fn series(
        &self,
        ctx: &Context<'_>,
    ) -> Result<
        Connection<
            u64,
            Series,
            EmptyFields,
            AnimeSeriesEdgeFields,
            AnimeSeriesConnection,
            AnimeSeriesEdge,
        >,
    > {
        let loader = ctx.data::<DataLoader<AnimeSeriesLoader>>()?;

        let rows = loader.load_one(self.id).await?.unwrap_or_default();

        let mut connection = Connection::with_additional_fields(false, false, EmptyFields);

        for (pivot, series) in rows {
            connection.edges.push(Edge::with_additional_fields(
                series.id,
                series.into(),
                AnimeSeriesEdgeFields {
                    created_at: pivot.created_at,
                    updated_at: pivot.updated_at,
                },
            ));
        }

        Ok(connection)
    }

    async fn studios(
        &self,
        ctx: &Context<'_>,
    ) -> Result<
        Connection<
            u64,
            Studio,
            EmptyFields,
            AnimeStudioEdgeFields,
            AnimeStudioConnection,
            AnimeStudioEdge,
        >,
    > {
        let loader = ctx.data::<DataLoader<AnimeStudiosLoader>>()?;

        let rows = loader.load_one(self.id).await?.unwrap_or_default();

        let mut connection = Connection::with_additional_fields(false, false, EmptyFields);

        for (pivot, studio) in rows {
            connection.edges.push(Edge::with_additional_fields(
                studio.id,
                studio.into(),
                AnimeStudioEdgeFields {
                    created_at: pivot.created_at,
                    updated_at: pivot.updated_at,
                },
            ));
        }

        Ok(connection)
    }
}

impl From<anime::Model> for Anime {
    fn from(model: anime::Model) -> Self {
        let title = AnimeTitle::from(&model);
        Self {
            id: model.id,
            name: title.romaji.clone(),
            format: model.format.map(Into::into),
            format_localized: model.format.map(|f| format!("{:?}", f)),
            season: model.season.map(Into::into),
            season_localized: model.season.map(|s| format!("{:?}", s)),
            slug: model.slug,
            synopsis: model.synopsis,
            title,
            year: model.year,
        }
    }
}
