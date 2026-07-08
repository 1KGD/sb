use bevy_ecs::prelude::*;
use macroquad::prelude::*;

use starbloom_base::*;
use starbloom_camera::*;

#[derive(Component, Default)]
#[require(Position)]
pub struct Player();

#[derive(Component, Default)]
#[require(Player)]
pub struct LocalPlayer();

pub struct PlayerPlugin();

impl Plugin for PlayerPlugin {
    fn create(world: &mut World, schedule: &mut Schedule) {
        schedule.add_systems((update_player, render_player));
        world.spawn(LocalPlayer());
    }
}

fn render_player(query: Query<&Position, With<Player>>) {
    for position in query {
        draw_rectangle(position.x, position.y, 20., 20., BLACK);
    }
}

fn update_player(
    mut query: Query<&mut Position, With<LocalPlayer>>,
    mut camera: ResMut<MainCamera>,
) {
    if let Ok(mut position) = query.single_mut() {
        if is_key_down(KeyCode::Down) {
            position.y += 100. * get_frame_time();
        }

        if is_key_down(KeyCode::Up) {
            position.y -= 100. * get_frame_time();
        }

        let old_pos: Vec2 = camera.position.clone();

        camera.position += (position.as_vec2() - old_pos) / 10.;
    }
}
