use animethemes_graphql_rust::{
    entities::content::{anime, animetheme, animethemeentry},
    enums::content::{
        animeformat::AnimeFormat as AnimeFormatEnum, themetype::ThemeType as ThemeTypeEnum,
    },
};
use async_graphql::{Context, InputObject, Object, Result};
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, JoinType, Order, QueryFilter, QueryOrder,
    QuerySelect, RelationTrait, sea_query::Expr,
};

use crate::graphql::{
    enums::content::{animeformat::AnimeFormat, themetype::ThemeType},
    types::content::animetheme::AnimeTheme,
};

#[derive(InputObject, Default)]
struct AnimeThemeShuffleInput {
    r#type: Option<Vec<ThemeType>>,
    format: Option<AnimeFormat>,
    year_gte: Option<i16>,
    year_lte: Option<i16>,
    spoiler: Option<bool>,
}

#[derive(Default)]
pub struct AnimeThemeQuery;

#[Object]
impl AnimeThemeQuery {
    async fn animetheme_shuffle(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = 15)] first: u64,
        input: Option<AnimeThemeShuffleInput>,
    ) -> Result<Vec<AnimeTheme>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let input = input.unwrap_or_default();

        let mut query = animetheme::Entity::find()
            .join(JoinType::InnerJoin, animetheme::Relation::Anime.def())
            .join(
                JoinType::InnerJoin,
                animetheme::Relation::Animethemeentry.def(),
            );

        if let Some(r#type) = input.r#type {
            query = query.filter(
                animetheme::Column::Type.is_in(r#type.into_iter().map(ThemeTypeEnum::from)),
            );
        }

        if let Some(format) = input.format {
            query = query.filter(anime::Column::Format.eq(AnimeFormatEnum::from(format)));
        }

        if let Some(year_gte) = input.year_gte {
            query = query.filter(anime::Column::Year.gte(year_gte));
        }

        if let Some(year_lte) = input.year_lte {
            query = query.filter(anime::Column::Year.lte(year_lte));
        }

        if let Some(spoiler) = input.spoiler {
            query = query.filter(animethemeentry::Column::Spoiler.eq(spoiler));
        }

        let themes = query
            .distinct()
            .order_by(Expr::cust("RAND()"), Order::Asc)
            .limit(first)
            .all(db)
            .await?;

        Ok(themes.into_iter().map(|m| m.into()).collect())
    }
}
