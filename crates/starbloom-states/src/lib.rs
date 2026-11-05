use bevy_ecs::prelude::*;
mod client;

pub use crate::client::*;

pub trait StateSetManager<T: SystemSet>: Resource {
    fn configure(&self, schedule: &mut Schedule, state: T) -> &Self;
    fn switch(&mut self, state: T);
}
