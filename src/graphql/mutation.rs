use async_graphql::MergedObject;

use crate::graphql::mutations::playlist::PlaylistMutation;

#[derive(MergedObject, Default)]
pub struct Mutation(
    //RootMutation,
    PlaylistMutation,
);

// #[derive(Default)]
// struct RootMutation;

// #[Object]
// impl RootMutation;
