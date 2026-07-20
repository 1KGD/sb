use bevy_ecs::prelude::*;
use glam::prelude::*;
use raylib::prelude::*;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub trait Plugin {
    fn create(world: &mut World, schedule: &mut Schedule);
}

#[derive(Resource)]
pub struct RenderContext(pub RaylibHandle);

#[derive(Default, Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn as_vec2(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    pub fn from_vec2(&mut self, vec: Vec2) {
        self.x = vec.x;
        self.y = vec.y;
    }
}
