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

impl Position {
    pub fn as_vec2(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}
