use bevy::prelude::*;
use std::fmt::Debug;
use std::hash::Hash;
use bevy::ecs::component::{Component, ComponentDescriptor};
use bevy::ecs::schedule::SystemDescriptor;
use bevy::reflect::GetTypeRegistration;

use crate::WindTunnel;

pub trait WindTunnelExt {
    fn add_state<T: Component + Debug + Clone + Eq + Hash>(self, state: T) -> WindTunnel;
    fn init_resource<T: Default + Send + Sync + 'static>(self) -> WindTunnel;
    fn insert_resource<T: Send + Sync + 'static>(self, res: T) -> WindTunnel;
    fn add_plugins<P: PluginGroup>(self, plugins: P) -> WindTunnel;
    fn add_plugin<P: Plugin>(self, plugin: P) -> WindTunnel;
    fn add_stage<S: Stage>(self, label: impl StageLabel, stage: S) -> WindTunnel;
    fn add_startup_system<S: Into<SystemDescriptor>>(self, sys: S) -> WindTunnel;
    fn add_system<S: Into<SystemDescriptor>>(self, sys: S) -> WindTunnel;
    fn add_system_set(self, system_set: SystemSet) -> WindTunnel;
    fn add_event<E: Component>(self) -> WindTunnel;
    fn register_component(self, descriptor: ComponentDescriptor) -> WindTunnel;
    fn register_type<T: GetTypeRegistration>(self) -> WindTunnel;
}

impl WindTunnelExt for WindTunnel {
    fn add_state<T: Component + Debug + Clone + Eq + Hash>(mut self, state: T) -> WindTunnel {
        self.builder.add_state(state);
        self
    }
    fn init_resource<T: Default + Send + Sync + 'static>(mut self) -> WindTunnel {
        self.builder.init_resource::<T>();
        self
    }
    fn insert_resource<T: Send + Sync + 'static>(mut self, res: T) -> WindTunnel {
        self.builder.insert_resource(res);
        self
    }
    fn add_plugins<P: PluginGroup>(mut self, plugins: P) -> WindTunnel {
       self.builder.add_plugins(plugins);
       self
   }
   fn add_plugin<P: Plugin>(mut self, plugin: P) -> WindTunnel {
       self.builder.add_plugin(plugin);
       self
   }
   fn add_stage<S: Stage>(mut self, label: impl StageLabel, stage: S) -> WindTunnel {
       self.builder.add_stage(label, stage);
       self
   }

   fn add_system<S: Into<SystemDescriptor>>(mut self, sys: S) -> WindTunnel {
       self.builder.add_system(sys);
       self
   }

   fn add_startup_system<S: Into<SystemDescriptor>>(mut self, sys: S) -> WindTunnel {
        self.builder.add_startup_system(sys);
        self
   }

   fn add_system_set(mut self, system_set: SystemSet) -> WindTunnel {
       self.builder.add_system_set(system_set);
       self
   }

   fn add_event<T: Component>(mut self) -> WindTunnel {
       self.builder.add_event::<T>();
       self
   }
   fn register_component(mut self, descriptor: ComponentDescriptor) -> WindTunnel {
      self.builder.register_component(descriptor);
      self
   }

   fn register_type<T: GetTypeRegistration>(mut self) -> WindTunnel {
       self.builder.register_type::<T>();
       self
   }
}
