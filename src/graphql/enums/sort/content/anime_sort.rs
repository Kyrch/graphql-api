use animethemes_graphql_rust::entities::content::anime;
use async_graphql::Enum;
use sea_orm::{Order, QueryOrder, Select, sea_query::Expr};

use crate::graphql::enums::sort::GraphQLSort;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum AnimeSort {
    Id,
    IdDesc,
    TitleRomaji,
    TitleRomajiDesc,
    TitleEnglish,
    TitleEnglishDesc,
    TitleNative,
    TitleNativeDesc,
    Year,
    YearDesc,
    CreatedAt,
    CreatedAtDesc,
    UpdatedAt,
    UpdatedAtDesc,
    Random,
}

impl GraphQLSort<anime::Entity> for AnimeSort {
    fn apply_sort(&self, query: Select<anime::Entity>) -> Select<anime::Entity> {
        match self {
            AnimeSort::Id => query.order_by(anime::Column::Id, Order::Asc),
            AnimeSort::IdDesc => query.order_by(anime::Column::Id, Order::Desc),
            AnimeSort::TitleRomaji => query.order_by(anime::Column::Title, Order::Asc),
            AnimeSort::TitleRomajiDesc => query.order_by(anime::Column::Title, Order::Desc),
            AnimeSort::TitleEnglish => query.order_by(anime::Column::TitleEnglish, Order::Asc),
            AnimeSort::TitleEnglishDesc => query.order_by(anime::Column::TitleEnglish, Order::Desc),
            AnimeSort::TitleNative => query.order_by(anime::Column::TitleNative, Order::Asc),
            AnimeSort::TitleNativeDesc => query.order_by(anime::Column::TitleNative, Order::Desc),
            AnimeSort::Year => query.order_by(anime::Column::Year, Order::Asc),
            AnimeSort::YearDesc => query.order_by(anime::Column::Year, Order::Desc),
            AnimeSort::CreatedAt => query.order_by(anime::Column::CreatedAt, Order::Asc),
            AnimeSort::CreatedAtDesc => query.order_by(anime::Column::CreatedAt, Order::Desc),
            AnimeSort::UpdatedAt => query.order_by(anime::Column::UpdatedAt, Order::Asc),
            AnimeSort::UpdatedAtDesc => query.order_by(anime::Column::UpdatedAt, Order::Desc),
            AnimeSort::Random => query.order_by(Expr::cust("RAND()"), Order::Asc),
        }
    }
}
