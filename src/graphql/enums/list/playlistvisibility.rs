use async_graphql::Enum;

use crate::enums::list::playlistvisibility::PlaylistVisibility as PlaylistVisibilityEnum;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum PlaylistVisibility {
    Public,
    Private,
    Unlisted,
}

impl From<PlaylistVisibilityEnum> for PlaylistVisibility {
    fn from(value: PlaylistVisibilityEnum) -> Self {
        match value {
            PlaylistVisibilityEnum::Public => PlaylistVisibility::Public,
            PlaylistVisibilityEnum::Private => PlaylistVisibility::Private,
            PlaylistVisibilityEnum::Unlisted => PlaylistVisibility::Unlisted,
        }
    }
}
