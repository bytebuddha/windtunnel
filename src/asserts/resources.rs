use std::fmt::Debug;
use bevy::prelude::*;
use std::marker::PhantomData;
use bevy::ecs::component::Component;

use crate::prelude::*;

pub struct ResourceExists<T: Component> {
    _phantom: PhantomData<T>
}

impl <T: Component>Default for ResourceExists<T> {
    fn default() -> ResourceExists<T> {
        ResourceExists {
            _phantom: Default::default()
        }
    }
}

impl <T: Component>Assert for ResourceExists<T> {
    fn assert(&self, world: &World) -> AssertResponse {
        if world.contains_resource::<T>() {
            AssertResponse::Success
        } else {
            AssertResponse::Failure(format!(
                "World doesn not contain resource `{}`",
                pretty_type_name::pretty_type_name::<T>()
            ))
        }
    }

    fn assert_exclusive(&self, world: &mut World) -> AssertResponse {
        if world.contains_resource::<T>() {
            AssertResponse::Success
        } else {
            AssertResponse::Failure(format!(
                "World doesn not contain resource `{}`",
                pretty_type_name::pretty_type_name::<T>()
            ))
        }
    }
}

#[derive(Default)]
pub struct Resource<T: Component + Debug + PartialEq> {
    pub value: T
}

impl <T: Component + Debug + PartialEq>Assert for Resource<T> {
    fn assert(&self, world: &World) -> AssertResponse {
        if let Some(res) = world.get_resource::<T>() {
            if res == &self.value {
                AssertResponse::Success
            } else {
                AssertResponse::Failure(format!(
                    "Resource `{}` does not match\n\tExpected\n{:?}\n\tGot:{:?}",
                    pretty_type_name::pretty_type_name::<T>(),
                    self.value,
                    *res
                ))
            }
        } else {
            AssertResponse::Failure(format!(
                "Resource `{}` does not exists",
                pretty_type_name::pretty_type_name::<T>(),
            ))
        }
    }

    fn assert_exclusive(&self, world: &mut World) -> AssertResponse {
        if let Some(res) = world.get_resource::<T>() {
            if res == &self.value {
                AssertResponse::Success
            } else {
                AssertResponse::Failure(format!(
                    "Resource `{}` does not match\n\tExpected\n{:?}\n\tGot:{:?}",
                    pretty_type_name::pretty_type_name::<T>(),
                    self.value,
                    *res
                ))
            }
        } else {
            AssertResponse::Failure(format!(
                "Resource `{}` does not exists",
                pretty_type_name::pretty_type_name::<T>(),
            ))
        }
    }
}
