use bevy::prelude::*;
use crate::prelude::*;

pub struct Wait<A: Assert> {
    pub seconds: f64,
    pub inner: A
}

impl <A: Assert>Assert for Wait<A> {
    fn assert(&self, world: &World) -> AssertResponse {
        let time = world.get_resource::<Time>().unwrap();
        if time.seconds_since_startup() > self.seconds {
            self.inner.assert(world)
        } else {
            AssertResponse::Incomplete
        }
    }

    fn assert_exclusive(&self, world: &mut World) -> AssertResponse {
        let time = world.get_resource::<Time>().unwrap();
        if time.seconds_since_startup() > self.seconds {
            self.inner.assert_exclusive(world)
        } else {
            AssertResponse::Incomplete
        }
    }
}
