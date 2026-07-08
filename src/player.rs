use bevy_ecs::prelude::*;
use macroquad::prelude::*;

use starbloom_base::*;
use starbloom_camera::*;
use starbloom_map::*;

const PLAYER_SPEED: f32 = 200.;

#[derive(Component, Default)]
#[require(Position)]
pub struct Player();

#[derive(Component, Default)]
#[require(Player)]
pub struct LocalPlayer();

pub struct PlayerPlugin();

impl Plugin for PlayerPlugin {
    fn create(world: &mut World, schedule: &mut Schedule) {
        schedule.add_systems(update_player.before(prepare_camera));
        schedule.add_systems(render_player.after(render_chunks));
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
        let mut motion: Vec2 = Vec2::ZERO;

        if is_key_down(KeyCode::Down) {
            motion.y += 1.;
        }

        if is_key_down(KeyCode::Up) {
            motion.y -= 1.;
        }

        if is_key_down(KeyCode::Left) {
            motion.x -= 1.;
        }

        if is_key_down(KeyCode::Right) {
            motion.x += 1.;
        }

        let pos: Vec2 = position.as_vec2();

        if motion.distance_squared(Vec2::ZERO) != 0. {
            // don't use an expensive square root if you
            // don't need it
            position.from_vec2(pos + motion.normalize() * PLAYER_SPEED * get_frame_time());
        }
        let old_pos: Vec2 = camera.position.clone();

        camera.position += (position.as_vec2() - old_pos) / 10.;
    }
}
