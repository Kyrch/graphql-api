use async_graphql::{ComplexObject, Context, Enum, Result, SimpleObject, dataloader::DataLoader};

use crate::{
    entities::animetheme::animetheme::{self, ThemeType as ThemeTypeEnum},
    graphql::types::{
        animetheme::animethemeentry::animethemeentry::AnimeThemeEntry, song::Song,
        theme_group::ThemeGroup,
    },
    loaders::anime::{
        anime_theme_entries::AnimeThemeEntriesLoader,
        animetheme::{
            animetheme_group::AnimeThemeGroupLoader, animetheme_song::AnimeThemeSongLoader,
        },
    },
};

/// Represents an OP or ED sequence for an anime.
///
/// For example, the anime Bakemonogatari has five OP anime themes and one ED anime theme.
#[derive(SimpleObject)]
#[graphql(complex)]
pub struct AnimeTheme {
    /// The primary key of the resource
    pub id: u64,
    #[graphql(skip)]
    pub group_id: Option<u64>,
    /// The numeric ordering of the theme
    pub sequence: Option<i32>,
    #[graphql(skip)]
    pub song_id: Option<u64>,
    /// The slug that represents the anime theme.
    pub slug: String,
    /// The type of the sequence
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

    async fn song(&self, ctx: &Context<'_>) -> Result<Option<Song>> {
        let Some(song_id) = self.song_id else {
            return Ok(None);
        };

        let loader = ctx.data::<DataLoader<AnimeThemeSongLoader>>()?;

        Ok(loader.load_one(song_id).await?.map(Into::into))
    }

    async fn group(&self, ctx: &Context<'_>) -> Result<Option<ThemeGroup>> {
        let Some(group_id) = self.group_id else {
            return Ok(None);
        };

        let loader = ctx.data::<DataLoader<AnimeThemeGroupLoader>>()?;

        Ok(loader.load_one(group_id).await?.map(Into::into))
    }
}

impl From<animetheme::Model> for AnimeTheme {
    fn from(model: animetheme::Model) -> Self {
        Self {
            id: model.id,
            group_id: model.group_id,
            sequence: model.sequence,
            song_id: model.song_id,
            slug: model.slug,
            themetype: model.themetype.into(),
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum ThemeType {
    /// Opening
    OP,
    /// Ending
    ED,
    /// Insert Song
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
