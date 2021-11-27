use bevy::prelude::*;

use crate::prelude::*;

#[derive(Default)]
pub struct Not<A: Assert> {
    pub inner: A
}

impl <A: Assert>Assert for Not<A> {
    fn assert(&self, world: &World) -> AssertResponse {
        match self.inner.assert(world) {
            AssertResponse::Incomplete => AssertResponse::Incomplete,
            AssertResponse::Success => AssertResponse::Failure(format!("Expected a value")),
            AssertResponse::Failure(_) => AssertResponse::Success
        }
    }

    fn assert_exclusive(&self, world: &mut World) -> AssertResponse {
        match self.inner.assert_exclusive(world) {
            AssertResponse::Incomplete => AssertResponse::Incomplete,
            AssertResponse::Success => AssertResponse::Failure(format!("Expected a value")),
            AssertResponse::Failure(_) => AssertResponse::Success
        }
    }
}
