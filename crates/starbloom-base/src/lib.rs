use bevy_ecs::prelude::*;
use macroquad::prelude::*;

#[derive(Component)]
pub struct Position(pub Vec2);

impl Default for Position {
    fn default() -> Self {
        Self(Vec2::default())
    }
}
