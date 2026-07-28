use async_graphql::{
    ComplexObject, Context, Enum, OutputType, Result, SimpleObject,
    connection::{Connection, DefaultConnectionName, Edge, EdgeNameType, EmptyFields},
    dataloader::DataLoader,
};

use crate::{
    entities::anime::{self, AnimeFormat as AnimeFormatEnum, AnimeSeason as AnimeSeasonEnum},
    graphql::types::{animetheme::animetheme::AnimeTheme, series::Series, synonym::Synonym},
    loaders::anime::{
        anime_series::AnimeSeriesLoader, anime_synonyms::AnimeSynonymsLoader,
        anime_themes::AnimeThemesLoader,
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

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Anime {
    pub id: u64,
    pub format: AnimeFormat,
    pub season: AnimeSeason,
    pub slug: String,
    pub synopsis: Option<String>,
    pub title: AnimeTitle,
    pub year: i32,
}

#[derive(SimpleObject)]
pub struct SeriesEdgeFields {
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

pub struct AnimeSeriesEdge;

impl EdgeNameType for AnimeSeriesEdge {
    fn type_name<T: OutputType>() -> String {
        "AnimeSeriesEdge".to_string()
    }
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

    async fn series(
        &self,
        ctx: &Context<'_>,
    ) -> Result<
        Connection<
            u64,
            Series,
            EmptyFields,
            SeriesEdgeFields,
            DefaultConnectionName,
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
                SeriesEdgeFields {
                    created_at: pivot.created_at.map(|dt| dt.to_rfc3339()),
                    updated_at: pivot.updated_at.map(|dt| dt.to_rfc3339()),
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
            format: model.format.into(),
            season: model.season.into(),
            slug: model.slug,
            synopsis: model.synopsis,
            title,
            year: model.year,
        }
    }
}
