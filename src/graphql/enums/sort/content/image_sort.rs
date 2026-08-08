use animethemes_graphql_rust::entities::content::image;
use async_graphql::Enum;
use sea_orm::{Order, QueryOrder, Select, sea_query::Expr};

use crate::graphql::enums::sort::GraphQLSort;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum ImageSort {
    Id,
    IdDesc,
    CreatedAt,
    CreatedAtDesc,
    UpdatedAt,
    UpdatedAtDesc,
    Random,
}

impl GraphQLSort<image::Entity> for ImageSort {
    fn apply_sort(&self, query: Select<image::Entity>) -> Select<image::Entity> {
        match self {
            ImageSort::Id => query.order_by(image::Column::Id, Order::Asc),
            ImageSort::IdDesc => query.order_by(image::Column::Id, Order::Desc),
            ImageSort::CreatedAt => query.order_by(image::Column::CreatedAt, Order::Asc),
            ImageSort::CreatedAtDesc => query.order_by(image::Column::CreatedAt, Order::Desc),
            ImageSort::UpdatedAt => query.order_by(image::Column::UpdatedAt, Order::Asc),
            ImageSort::UpdatedAtDesc => query.order_by(image::Column::UpdatedAt, Order::Desc),
            ImageSort::Random => query.order_by(Expr::cust("RAND()"), Order::Asc),
        }
    }
}
