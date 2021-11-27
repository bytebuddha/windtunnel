use std::fmt::Debug;
use bevy::prelude::*;
use std::ops::{Deref, DerefMut};
use bevy::ecs::component::Component;
//use bevy::ecs::schedule::SystemMeta;
use bevy::ecs::system::{SystemParam, SystemParamFetch, SystemParamState, SystemState};

pub struct Required<'a, T: Component>(&'a mut T);

// SAFE: Required only accesses internal state
//unsafe impl<T: Component> SystemParamFetch for RequiredState<T> {}

impl<'a, T: Component> Debug for Required<'a, T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Required").field(&self.0).finish()
    }
}

impl<'a, T: Component> Deref for Required<'a, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a, T: Component> DerefMut for Required<'a, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

/// The [`SystemParamState`] of [`Required`].
pub struct RequiredState<T: Component>(T);

impl<'a, T: Component> SystemParam for Required<'a, T> {
    type Fetch = RequiredState<T>;
}

// SAFE: only local state is accessed
unsafe impl<T: Component> SystemParamState for RequiredState<T> {
    type Config = Option<T>;

    fn init(_world: &mut World, _system_meta: &mut SystemState, config: Self::Config) -> Self {
        Self(config.expect("Required must be initialized using config!"))
    }

    fn default_config() -> Option<T> {
        None
    }
}

impl<'a, T: Component> SystemParamFetch<'a> for RequiredState<T> {
    type Item = Required<'a, T>;

    #[inline]
    unsafe fn get_param(
        state: &'a mut Self,
        _system_meta: &SystemState,
        _world: &'a World,
        _change_tick: u32,
    ) -> Self::Item {
        Required(&mut state.0)
    }
}
