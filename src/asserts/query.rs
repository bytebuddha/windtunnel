use std::marker::PhantomData;
use bevy::prelude::*;
use crate::prelude::*;
use bevy::ecs::query::WorldQuery;

pub struct QueryIsSome<Q: WorldQuery> {
    q: PhantomData<Q>
}

impl <Q: WorldQuery>Default for QueryIsSome<Q> {
    fn default() -> QueryIsSome<Q> {
        QueryIsSome { q: Default::default() }
    }
}

impl <Q: WorldQuery + Send + Sync + 'static>Assert for QueryIsSome<Q> {
    fn assert(&self, _: &World) -> AssertResponse {
        panic!(
            "{} can only be used exclusivily",
            pretty_type_name::pretty_type_name::<Self>()
        );
    }

    fn assert_exclusive(&self, world: &mut World) -> AssertResponse {
        let mut query = world.query::<Q>();
        if query.iter_mut(world).next().is_some() {
            AssertResponse::Success
        } else {
            AssertResponse::Failure(format!(
                "Query `{}` returned no results",
                pretty_type_name::pretty_type_name::<Self>()
            ))

        }
    }
}
