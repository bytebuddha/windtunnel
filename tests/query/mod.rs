use bevy::prelude::*;
use windtunnel::prelude::*;

#[test]
fn simple_query() {
    struct Marker;
    fn add_entity(mut commands: Commands) {
        commands.spawn().insert(Marker);
    }
    WindTunnel::default()
        .add_startup_system(add_entity.system())
        .assert_query_is_some::<&Marker>()
        .assert_entity_count(1)
        .run();
}

#[test]
fn simple_query_empty() {
    struct Marker;
    WindTunnel::default()
        .assert_query_is_none::<&Marker>()
        .run();
}
