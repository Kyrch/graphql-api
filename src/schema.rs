use async_graphql::{
    EmptyMutation, EmptySubscription, Schema, dataloader::DataLoader, http::GraphiQLSource,
};

use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{extract::State, response::Html};
use sea_orm::DatabaseConnection;

use crate::graphql::{
    loaders::{
        anime::{
            anime_series::AnimeSeriesLoader,
            anime_synonyms::AnimeSynonymsLoader,
            anime_theme_entries::AnimeThemeEntriesLoader,
            anime_themes::AnimeThemesLoader,
            animetheme::{
                animetheme_group::AnimeThemeGroupLoader, animetheme_song::AnimeThemeSongLoader,
                animethemeentry::animethemeentry_videos::AnimeThemeEntryVideosLoader,
            },
        },
        artist::artist_performances::ArtistPerformancesLoader,
        performance::{
            performance_artist::PerformanceArtistLoader,
            performance_member::PerformanceMemberLoader, performance_song::PerformanceSongLoader,
        },
        song::song_performances::SongPerformancesLoader,
        video::{video_audio::VideoAudioLoader, video_script::VideoScriptLoader},
    },
    query::Query,
};

pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub fn create_schema(db: DatabaseConnection) -> AppSchema {
    let anime_synonyms_loader =
        DataLoader::new(AnimeSynonymsLoader { db: db.clone() }, tokio::spawn);

    let anime_themes_loader = DataLoader::new(AnimeThemesLoader { db: db.clone() }, tokio::spawn);

    let animetheme_song_loader =
        DataLoader::new(AnimeThemeSongLoader { db: db.clone() }, tokio::spawn);

    let animetheme_group_loader =
        DataLoader::new(AnimeThemeGroupLoader { db: db.clone() }, tokio::spawn);

    let anime_theme_entries_loader =
        DataLoader::new(AnimeThemeEntriesLoader { db: db.clone() }, tokio::spawn);

    let anime_theme_entry_videos_loader =
        DataLoader::new(AnimeThemeEntryVideosLoader { db: db.clone() }, tokio::spawn);

    let anime_series_loader = DataLoader::new(AnimeSeriesLoader { db: db.clone() }, tokio::spawn);

    let artist_performances_loader =
        DataLoader::new(ArtistPerformancesLoader { db: db.clone() }, tokio::spawn);

    let song_performances_loader =
        DataLoader::new(SongPerformancesLoader { db: db.clone() }, tokio::spawn);

    let performance_artist_loader =
        DataLoader::new(PerformanceArtistLoader { db: db.clone() }, tokio::spawn);

    let performance_member_loader =
        DataLoader::new(PerformanceMemberLoader { db: db.clone() }, tokio::spawn);

    let performance_song_loader =
        DataLoader::new(PerformanceSongLoader { db: db.clone() }, tokio::spawn);

    let video_audio_loader = DataLoader::new(VideoAudioLoader { db: db.clone() }, tokio::spawn);

    let video_script_loader = DataLoader::new(VideoScriptLoader { db: db.clone() }, tokio::spawn);

    Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(db)
        .data(anime_synonyms_loader)
        .data(anime_themes_loader)
        .data(animetheme_song_loader)
        .data(animetheme_group_loader)
        .data(anime_theme_entries_loader)
        .data(anime_theme_entry_videos_loader)
        .data(anime_series_loader)
        .data(artist_performances_loader)
        .data(song_performances_loader)
        .data(performance_artist_loader)
        .data(performance_member_loader)
        .data(performance_song_loader)
        .data(video_audio_loader)
        .data(video_script_loader)
        .finish()
}

pub async fn graphql_handler(
    State(schema): State<AppSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

pub async fn graphiql() -> Html<String> {
    Html(GraphiQLSource::build().endpoint("/").finish())
}
