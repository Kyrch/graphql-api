use async_graphql::{ComplexObject, Context, Enum, Result, SimpleObject, dataloader::DataLoader};

use crate::{
    entities::animetheme::animetheme::{self, ThemeType as ThemeTypeEnum},
    graphql::types::animetheme::animethemeentry::animethemeentry::AnimeThemeEntry,
    loaders::anime::anime_theme_entries::AnimeThemeEntriesLoader,
};

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct AnimeTheme {
    pub id: u64,
    pub sequence: Option<i32>,
    pub slug: String,
    #[graphql(name = "type")]
    pub themetype: ThemeType,
}

#[ComplexObject]
impl AnimeTheme {
    async fn animethemeentries(&self, ctx: &Context<'_>) -> Result<Vec<AnimeThemeEntry>> {
        let loader = ctx.data::<DataLoader<AnimeThemeEntriesLoader>>()?;

        let models = loader.load_one(self.id).await?.unwrap_or_default();

        Ok(models.into_iter().map(AnimeThemeEntry::from).collect())
    }
}

impl From<animetheme::Model> for AnimeTheme {
    fn from(model: animetheme::Model) -> Self {
        Self {
            id: model.id,
            sequence: model.sequence,
            slug: model.slug,
            themetype: model.themetype.into(),
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum ThemeType {
    OP,
    ED,
    IN,
}

impl From<ThemeTypeEnum> for ThemeType {
    fn from(value: ThemeTypeEnum) -> Self {
        match value {
            ThemeTypeEnum::OP => ThemeType::OP,
            ThemeTypeEnum::ED => ThemeType::ED,
            ThemeTypeEnum::IN => ThemeType::IN,
        }
    }
}
