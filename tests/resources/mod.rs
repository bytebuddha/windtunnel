use bevy::prelude::*;
use windtunnel::prelude::*;

#[test]
fn exists() {
    WindTunnel::default()
        .init_resource::<u8>()
        .assert_resource_exists::<u8>()
        .run();
}

#[test]
#[should_panic]
fn exists_fail() {
    WindTunnel::default()
        .assert_resource_exists::<u8>()
        .run();
}

#[test]
fn insert() {
    WindTunnel::default()
        .insert_resource(24u8)
        .assert_resource_equals(24u8)
        .run();
}

#[test]
#[should_panic]
fn insert_fail() {
    WindTunnel::default()
        .assert_resource_equals(24u8)
        .run();
}

#[test]
fn delayed_insert() {
    fn delayed_insert_system(mut commands: Commands, res: Res<Time>) {
        if res.seconds_since_startup() > 2.0 {
            commands.insert_resource(24u8);
        }
    }
    WindTunnel::default()
        .add_plugins(MinimalPlugins)
        .add_system(delayed_insert_system.system())
        .wait_then_assert::<Not<ResourceExists<u8>>>(1.0)
        .wait_then_assert::<ResourceExists<u8>>(3.0)
        .run()
}

#[test]
fn delayed_removal() {
    fn delayed_removal_system(mut commands: Commands, res: Res<Time>) {
        if res.seconds_since_startup() > 2.0 {
            commands.remove_resource::<u8>();
        }
    }
    WindTunnel::default()
        .add_plugins(MinimalPlugins)
        .init_resource::<u8>()
        .add_system(delayed_removal_system.system())
        .wait_then_assert::<ResourceExists<u8>>(1.0)
        .wait_then_assert::<Not<ResourceExists<u8>>>(3.0)
        .run()
}
