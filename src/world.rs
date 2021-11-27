use bevy::ecs::{
        world::World,
        query::ReadOnlyFetch,
        system::{SystemParam, SystemState, SystemParamState, SystemParamFetch},
};

pub struct RawWorld<'w> {
    pub world: &'w World
}

/// SAFE: only reads world
unsafe impl ReadOnlyFetch for WorldState {}

/// The [`SystemParamState`] of [`&World`].
pub struct WorldState;

impl<'w, 's> SystemParam for RawWorld<'w> {
    type Fetch = WorldState;
}

unsafe impl<'w, 's> SystemParamState for WorldState {
    type Config = ();

    fn init(_world: &mut World, _: &mut SystemState, _config: Self::Config) -> Self {

        WorldState
    }

    fn default_config() -> Self::Config {}
}

impl<'w> SystemParamFetch<'w> for WorldState {
    type Item = RawWorld<'w>;
    unsafe fn get_param(
        _state: &'w mut Self,
        _system_meta: &'w SystemState,
        world: &'w World,
        _change_tick: u32,
    ) -> Self::Item {
        RawWorld { world }
    }
}
