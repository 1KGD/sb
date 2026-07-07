use bevy_ecs::prelude::*;
use macroquad::prelude::*;

#[derive(Default, Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}
