use bevy::prelude::*;
use bevy::ecs::component::Component;
use type_map::concurrent::TypeMap;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum AssertResponse {
    Incomplete,
    Success,
    Failure(String)
}

#[derive(Default)]
pub struct ExclusiveAssertions(pub TypeMap);

pub trait Assert: Component {
    fn assert(&self, world: &World) -> AssertResponse;
    fn assert_exclusive(&self, world: &mut World) -> AssertResponse;
}


#[derive(Default)]
pub struct Assertions {
    pub total: usize,
    pub completed: usize
}
