use bevy::prelude::*;

use crate::prelude::*;

pub struct EntityCount {
    pub count: u32
}

impl Assert for EntityCount {
    fn assert(&self, world: &World) -> AssertResponse {
        let count = world.entities().len();
        if self.count == count {
            AssertResponse::Success
        } else {
            AssertResponse::Failure(format!(
                "World does not contain `{}` entities, contains `{}`",
                self.count,
                count
            ))
        }
    }

    fn assert_exclusive(&self, world: &mut World) -> AssertResponse {
        let count = world.entities().len();
        if self.count == count {
            AssertResponse::Success
        } else {
            AssertResponse::Failure(format!(
                "World does not contain `{}` entities, contains `{}`",
                self.count,
                count
            ))
        }
    }
}
