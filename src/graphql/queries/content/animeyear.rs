use std::collections::BTreeMap;

use animethemes_graphql_rust::{
    entities::content::anime,
    enums::{LocalizedEnum, content::animeseason::AnimeSeason as AnimeSeasonEntity},
};
use async_graphql::{
    ComplexObject, Context, Object, Result, SimpleObject,
    connection::{Connection, EmptyFields},
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect};

use crate::graphql::{
    enums::content::animeseason::AnimeSeason,
    inputs::pagination_input::PaginationInput,
    queries::content::anime::{AnimeFilterInput, AnimeQuery},
    types::content::anime::Anime,
};

/// The anime year response type, grouped by season.
#[derive(SimpleObject)]
#[graphql(complex)]
pub struct AnimeYear {
    /// The year of the AnimeYear type.
    year: i16,

    /// Internal value. It is not exposed in GraphQL.
    #[graphql(skip)]
    available_seasons: Vec<AnimeSeason>,
}

#[ComplexObject]
impl AnimeYear {
    /// Object that references the season year queried.
    async fn season(&self, season: AnimeSeason) -> Option<AnimeYearSeason> {
        if !self.available_seasons.contains(&season) {
            return None;
        }

        let season_localized = AnimeSeasonEntity::from(season).localize().to_string();

        Some(AnimeYearSeason {
            season,
            season_localized,
            year: self.year,
        })
    }

    /// The available seasons of the year.
    async fn seasons(&self) -> Vec<AnimeYearSeason> {
        self.available_seasons
            .iter()
            .cloned()
            .map(|season| {
                let season_localized = AnimeSeasonEntity::from(season).localize().to_string();
                AnimeYearSeason {
                    season,
                    season_localized,
                    year: self.year,
                }
            })
            .collect()
    }
}

/// The anime year season type.
#[derive(SimpleObject)]
#[graphql(complex)]
pub struct AnimeYearSeason {
    /// The season of the anime year.
    season: AnimeSeason,

    /// The formatted string value of the season field.
    season_localized: String,

    /// Used internally by the `anime` resolver.
    #[graphql(skip)]
    year: i16,
}

#[ComplexObject]
impl AnimeYearSeason {
    async fn anime(
        &self,
        ctx: &Context<'_>,
        pagination: Option<PaginationInput>,
        filter: Option<AnimeFilterInput>,
    ) -> Result<Connection<u64, Anime, EmptyFields, EmptyFields>> {
        let mut filter = filter.unwrap_or_default();

        filter.animeyear_year = Some(self.year);
        filter.animeyear_season = Some(self.season.clone().into());

        AnimeQuery::default()
            .anime_connection(ctx, pagination, Some(filter))
            .await
    }
}

#[derive(Default)]
pub struct AnimeYearQuery;

#[Object]
impl AnimeYearQuery {
    /// Returns a list of years grouped by its seasons.
    async fn animeyears(
        &self,
        ctx: &Context<'_>,
        year: Option<Vec<i16>>,
    ) -> Result<Vec<AnimeYear>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let mut query = anime::Entity::find()
            .select_only()
            .column(anime::Column::Year)
            .column(anime::Column::Season)
            .filter(anime::Column::Year.is_not_null())
            .filter(anime::Column::Season.is_not_null())
            .distinct()
            .order_by_asc(anime::Column::Year)
            .order_by_asc(anime::Column::Season);

        if let Some(years) = year.filter(|years| !years.is_empty()) {
            query = query.filter(anime::Column::Year.is_in(years));
        }

        let rows = query
            .into_tuple::<(Option<i16>, Option<AnimeSeasonEntity>)>()
            .all(db)
            .await?;

        let mut grouped: BTreeMap<i16, Vec<AnimeSeason>> = BTreeMap::new();

        for (year, season) in rows {
            let (Some(year), Some(season)) = (year, season) else {
                continue;
            };

            let season: AnimeSeason = season.into();

            let seasons = grouped.entry(year).or_default();

            if !seasons.contains(&season) {
                seasons.push(season);
            }
        }

        Ok(grouped
            .into_iter()
            .map(|(year, available_seasons)| AnimeYear {
                year,
                available_seasons,
            })
            .collect())
    }
}
