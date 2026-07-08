use bevy_ecs::prelude::*;
use macroquad::prelude::*;

pub trait Plugin {
    fn create(world: &mut World, schedule: &mut Schedule);
}

#[derive(Default, Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}
