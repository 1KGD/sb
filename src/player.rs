use bevy_ecs::prelude::*;
use macroquad::prelude::*;

use starbloom_base::*;
use starbloom_camera::*;
use starbloom_map::*;

const PLAYER_SPEED: f32 = 200.;

const PLAYER_NAME_FNT_SIZE: u16 = 20;

#[derive(Component, Default)]
#[require(Position)]
pub struct Player {
    pub name: String,
}

#[derive(Component, Default)]
#[require(Player)]
pub struct LocalPlayer();

pub struct PlayerPlugin();

impl Plugin for PlayerPlugin {
    fn create(world: &mut World, schedule: &mut Schedule) {
        schedule.add_systems(update_local_player.before(prepare_camera));
        schedule.add_systems(
            render_players
                .after(render_chunks)
                .before(prepare_ui_camera),
        );
        schedule.add_systems(
            render_player_names
                .after(render_players)
                .before(prepare_ui_camera),
        );
        world.spawn(LocalPlayer());
    }
}

fn render_players(query: Query<&Position, With<Player>>) {
    for position in query {
        draw_rectangle(position.x - 10., position.y - 10., 20., 20., BLACK);
    }
}

fn render_player_names(query: Query<(&Position, &Player), Without<LocalPlayer>>) {
    for (position, player) in query {
        let dims = measure_text(&player.name, None, PLAYER_NAME_FNT_SIZE, 1.);
        draw_text(
            &player.name,
            position.x - dims.width / 2.,
            position.y - 15.,
            PLAYER_NAME_FNT_SIZE as f32,
            WHITE,
        );
    }
}

fn update_local_player(
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
