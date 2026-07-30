use chrono::Utc;
use sea_orm::entity::prelude::*;

use crate::{
    entities::{
        SoftDeleteEntity,
        content::{anime, animetheme::animethemeentry::animethemeentry, song, themegroup},
    },
    enums::content::themetype::ThemeType,
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "anime_themes")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "theme_id")]
    pub id: u64,
    pub anime_id: u64,
    pub group_id: Option<u64>,
    pub sequence: Option<i32>,
    pub slug: String,
    pub song_id: Option<u64>,
    pub r#type: ThemeType,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: Option<chrono::DateTime<Utc>>,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: Option<chrono::DateTime<Utc>>,
    #[sea_orm(column_type = "Timestamp")]
    pub deleted_at: Option<chrono::DateTime<Utc>>,
}

impl SoftDeleteEntity for Entity {
    fn deleted_at_column() -> Self::Column {
        Column::DeletedAt
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "anime::Entity",
        from = "Column::AnimeId",
        to = "anime::Column::Id"
    )]
    Anime,

    #[sea_orm(has_many = "animethemeentry::Entity")]
    AnimeThemeEntries,

    #[sea_orm(
        belongs_to = "song::Entity",
        from = "Column::SongId",
        to = "song::Column::Id"
    )]
    Song,

    #[sea_orm(
        belongs_to = "themegroup::Entity",
        from = "Column::GroupId",
        to = "themegroup::Column::Id"
    )]
    ThemeGroup,
}

impl Related<anime::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Anime.def()
    }
}

impl Related<animethemeentry::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AnimeThemeEntries.def()
    }
}

impl Related<song::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Song.def()
    }
}

impl Related<themegroup::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ThemeGroup.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
