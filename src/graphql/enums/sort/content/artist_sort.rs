use animethemes_graphql_rust::entities::content::artist;
use async_graphql::Enum;
use sea_orm::{Order, QueryOrder, Select, sea_query::Expr};

use crate::graphql::enums::sort::GraphQLSort;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum ArtistSort {
    Id,
    IdDesc,
    NameMain,
    NameMainDesc,
    NameNative,
    NameNativeDesc,
    CreatedAt,
    CreatedAtDesc,
    UpdatedAt,
    UpdatedAtDesc,
    Random,
}

impl GraphQLSort<artist::Entity> for ArtistSort {
    fn apply_sort(&self, query: Select<artist::Entity>) -> Select<artist::Entity> {
        match self {
            ArtistSort::Id => query.order_by(artist::Column::Id, Order::Asc),
            ArtistSort::IdDesc => query.order_by(artist::Column::Id, Order::Desc),
            ArtistSort::NameMain => query.order_by(artist::Column::Name, Order::Asc),
            ArtistSort::NameMainDesc => query.order_by(artist::Column::Name, Order::Desc),
            ArtistSort::NameNative => query.order_by(artist::Column::NameNative, Order::Asc),
            ArtistSort::NameNativeDesc => query.order_by(artist::Column::NameNative, Order::Desc),
            ArtistSort::CreatedAt => query.order_by(artist::Column::CreatedAt, Order::Asc),
            ArtistSort::CreatedAtDesc => query.order_by(artist::Column::CreatedAt, Order::Desc),
            ArtistSort::UpdatedAt => query.order_by(artist::Column::UpdatedAt, Order::Asc),
            ArtistSort::UpdatedAtDesc => query.order_by(artist::Column::UpdatedAt, Order::Desc),
            ArtistSort::Random => query.order_by(Expr::cust("RAND()"), Order::Asc),
        }
    }
}
