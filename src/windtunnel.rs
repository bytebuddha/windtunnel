use bevy::prelude::*;
use bevy::app::AppExit;

use crate::assert::{Assertions, Assert, AssertResponse, ExclusiveAssertions};

use crate::{
    world::RawWorld,
    required::Required
};

pub struct WindTunnel {
    pub(crate) builder: AppBuilder
}

impl Default for WindTunnel {
    fn default() -> WindTunnel {
        WindTunnel::new()
    }
}

impl WindTunnel {

    pub fn new() -> WindTunnel {
        let mut builder = AppBuilder::default();
        builder.init_resource::<Assertions>()
            .init_resource::<ExclusiveAssertions>()
            .add_system(check_progress.system());
        WindTunnel { builder }
    }

    pub fn assert<A: Assert>(mut self, assert: A) -> WindTunnel {
        self.builder.add_system(
            assert_system::<A>.system().config(|(ass, _)| {
                *ass = Some(assert);
            }).chain(assert_tracking_system.system())
        );
        let mut assertions = self.builder.app.world.get_resource_mut::<Assertions>().unwrap();
        assertions.total += 1;
        self
    }

    pub fn assert_exclusive<A: Assert>(mut self, assert: A) -> WindTunnel {
        self.builder.add_system(
            assert_system_exclusive::<A>.exclusive_system()
        );
        let mut exclusives = self.builder.app.world.get_resource_mut::<ExclusiveAssertions>().unwrap();
        exclusives.0.insert(assert);
        let mut assertions = self.builder.app.world.get_resource_mut::<Assertions>().unwrap();
        assertions.total += 1;
        self
    }

    pub fn run(mut self) {
        self.builder.run();
    }
}

impl AsRef<AppBuilder> for WindTunnel {
    fn as_ref(&self) -> &AppBuilder {
        &self.builder
    }
}

fn assert_system<A: Assert>(
    assert: Required<A>,
    world: RawWorld
) -> AssertResponse {
    assert.assert(world.world)
}

fn assert_system_exclusive<A: Assert>(world: &mut World) {
    let world_ptr = world as *mut _;
    let world_clone: &mut World = unsafe { &mut *world_ptr };
    let exclusives = world.get_resource_mut::<ExclusiveAssertions>().unwrap();
    match A::assert_exclusive(exclusives.0.get::<A>().unwrap(), world_clone) {
        AssertResponse::Success => {
            let mut assertions = world.get_resource_mut::<Assertions>().unwrap();
            assertions.completed += 1
        },
        AssertResponse::Failure(msg) => panic!("{}", msg),
        AssertResponse::Incomplete => {}
    }
}

fn assert_tracking_system(In(response): In<AssertResponse>, mut assertions: ResMut<Assertions>) {
    match response {
        AssertResponse::Success => assertions.completed += 1,
        AssertResponse::Failure(msg) => panic!("{}", msg),
        AssertResponse::Incomplete => {}
    }
}

fn check_progress(res: Res<Assertions>, mut exit: EventWriter<AppExit>) {
    if res.total == res.completed {
        exit.send(AppExit);
    } else if res.completed > res.total {
        error!("Expected number of tests less than the received results!");
    }
}
