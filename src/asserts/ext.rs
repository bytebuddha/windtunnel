use std::fmt::Debug;
use bevy::ecs::query::WorldQuery;
use bevy::ecs::component::Component;

use crate::prelude::*;

pub trait AssertionExt {
    fn assert_resource_exists<T: Component>(self) -> WindTunnel;
    fn assert_resource_equals<T: Component + PartialEq + Debug>(self, value: T) -> WindTunnel;
    fn wait_then_assert<A: Assert + Default>(self, seconds: f64) -> WindTunnel;
    fn wait_then_make_assert<A: Assert>(self, seconds: f64, inner: A) -> WindTunnel;
    fn assert_query_is_some<Q: WorldQuery + Send + Sync + 'static>(self) -> WindTunnel;
    fn assert_query_is_none<Q: WorldQuery + Send + Sync + 'static>(self) -> WindTunnel;
    fn assert_entity_count(self, count: u32) -> WindTunnel;
}

impl AssertionExt for WindTunnel {
    fn assert_resource_exists<T: Component>(self) -> WindTunnel {
        self.assert(ResourceExists::<T>::default())
    }
    fn assert_resource_equals<T: Component + PartialEq + Debug>(self, value: T) -> WindTunnel {
        self.assert(Resource::<T> { value })
    }
    fn wait_then_assert<A: Assert + Default>(self, seconds: f64) -> WindTunnel {
        self.assert(Wait::<A> { seconds, inner: Default::default() })
    }
    fn wait_then_make_assert<A: Assert>(self, seconds: f64, inner: A) -> WindTunnel {
        self.assert(Wait::<A> { seconds, inner })
    }
    fn assert_query_is_some<Q: WorldQuery + Send + Sync + 'static>(self) -> WindTunnel {
        self.assert_exclusive(QueryIsSome::<Q>::default())
    }
    fn assert_query_is_none<Q: WorldQuery + Send + Sync + 'static>(self) -> WindTunnel {
        self.assert_exclusive(Not { inner: QueryIsSome::<Q>::default() })
    }
    fn assert_entity_count(self, count: u32) -> WindTunnel {
        self.assert(EntityCount { count })
    }
}
