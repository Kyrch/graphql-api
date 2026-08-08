use animethemes_graphql_rust::entities::content::studio;
use async_graphql::Enum;
use sea_orm::{Order, QueryOrder, Select, sea_query::Expr};

use crate::graphql::enums::sort::GraphQLSort;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum StudioSort {
    Id,
    IdDesc,
    Name,
    NameDesc,
    CreatedAt,
    CreatedAtDesc,
    UpdatedAt,
    UpdatedAtDesc,
    Random,
}

impl GraphQLSort<studio::Entity> for StudioSort {
    fn apply_sort(&self, query: Select<studio::Entity>) -> Select<studio::Entity> {
        match self {
            StudioSort::Id => query.order_by(studio::Column::Id, Order::Asc),
            StudioSort::IdDesc => query.order_by(studio::Column::Id, Order::Desc),
            StudioSort::Name => query.order_by(studio::Column::Name, Order::Asc),
            StudioSort::NameDesc => query.order_by(studio::Column::Name, Order::Desc),
            StudioSort::CreatedAt => query.order_by(studio::Column::CreatedAt, Order::Asc),
            StudioSort::CreatedAtDesc => query.order_by(studio::Column::CreatedAt, Order::Desc),
            StudioSort::UpdatedAt => query.order_by(studio::Column::UpdatedAt, Order::Asc),
            StudioSort::UpdatedAtDesc => query.order_by(studio::Column::UpdatedAt, Order::Desc),
            StudioSort::Random => query.order_by(Expr::cust("RAND()"), Order::Asc),
        }
    }
}
