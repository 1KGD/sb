use bevy_ecs::prelude::*;
use egor::{input::*, math::*};

use starbloom_base::prelude::*;
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
        schedule.add_systems(update_local_player);
        schedule.add_systems(render_players.after(render_chunks));
        schedule.add_systems(render_player_names.after(render_players));
        world.spawn(LocalPlayer());
    }
}

fn render_players(query: Query<&Position, With<Player>>) {
    for position in query {}
}

fn render_player_names(
    query: Query<(&Position, &Player), Without<LocalPlayer>>,
    mut gfx: NonSendMut<GfxCmds>,
) {
    for (position, player) in query {
        gfx.insert(RenderCmd::PositionedText(
            player.name.to_owned(),
            position.as_vec2(),
        ));
    }
}

fn update_local_player(
    mut query: Query<&mut Position, With<LocalPlayer>>,
    mut camera: ResMut<MainCamera>,
    input: NonSend<Input>,
) {
    if let Ok(mut position) = query.single_mut() {
        let mut motion: Vec2 = Vec2::ZERO;

        if input.key_held(KeyCode::ArrowDown) {
            motion.y += 1.;
        }

        if input.key_held(KeyCode::ArrowUp) {
            motion.y -= 1.;
        }

        if input.key_held(KeyCode::ArrowLeft) {
            motion.x -= 1.;
        }

        if input.key_held(KeyCode::ArrowRight) {
            motion.x += 1.;
        }

        let pos: Vec2 = position.as_vec2();

        // Don't use an expensive square root if you don't need it.
        if motion.distance_squared(Vec2::ZERO) != 0. {
            position.from_vec2(
                pos + motion.normalize() * PLAYER_SPEED, /* * get_frame_time()*/
            );
        }
        let old_pos: Vec2 = camera.position.clone();

        camera.position += (position.as_vec2() - old_pos) / 10.;
    }
}
