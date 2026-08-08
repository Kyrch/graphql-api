use animethemes_graphql_rust::entities::content::series;
use async_graphql::Enum;
use sea_orm::{Order, QueryOrder, Select, sea_query::Expr};

use crate::graphql::enums::sort::GraphQLSort;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum SeriesSort {
    Id,
    IdDesc,
    TitleRomaji,
    TitleRomajiDesc,
    CreatedAt,
    CreatedAtDesc,
    UpdatedAt,
    UpdatedAtDesc,
    Random,
}

impl GraphQLSort<series::Entity> for SeriesSort {
    fn apply_sort(&self, query: Select<series::Entity>) -> Select<series::Entity> {
        match self {
            SeriesSort::Id => query.order_by(series::Column::Id, Order::Asc),
            SeriesSort::IdDesc => query.order_by(series::Column::Id, Order::Desc),
            SeriesSort::TitleRomaji => query.order_by(series::Column::Title, Order::Asc),
            SeriesSort::TitleRomajiDesc => query.order_by(series::Column::Title, Order::Desc),
            SeriesSort::CreatedAt => query.order_by(series::Column::CreatedAt, Order::Asc),
            SeriesSort::CreatedAtDesc => query.order_by(series::Column::CreatedAt, Order::Desc),
            SeriesSort::UpdatedAt => query.order_by(series::Column::UpdatedAt, Order::Asc),
            SeriesSort::UpdatedAtDesc => query.order_by(series::Column::UpdatedAt, Order::Desc),
            SeriesSort::Random => query.order_by(Expr::cust("RAND()"), Order::Asc),
        }
    }
}
