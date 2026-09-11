use starbloom_base::prelude::*;
use starbloom_camera::*;
use starbloom_input::prelude::*;
use starbloom_map::*;

const PLAYER_SPEED: f32 = 200.;

const PLAYER_NAME_FNT_SIZE: f32 = 20.;

#[derive(Component, Default)]
#[require(Position)]
pub struct Player;

#[derive(Component, Default)]
#[require(Player)]
pub struct LocalPlayer;

#[derive(Component, Default)]
#[require(Player)]
pub struct RemotePlayer {
    pub name: String,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn create(world: &mut World, schedule: &mut Schedule) {
        schedule.add_systems(update_local_player);
        schedule.add_systems(render_players.after(render_chunks));
        schedule.add_systems(render_player_names.after(render_players));
        world.spawn(LocalPlayer::default());
    }
}

fn render_players(
    query: Query<&Position, With<Player>>,
    main_camera: Res<MainCamera>,
    mut renderer: NonSendMut<Renderer>,
) {
    if let Some(ctx) = renderer.ctx() {
        for position in query {
            ctx.gfx
                .rect()
                .anchor(Anchor::Center)
                .at(main_camera.cam.world_to_screen(position.as_vec2()))
                .color(Color::RED);
        }
    }
}

fn render_player_names(
    query: Query<(&Position, &RemotePlayer), With<Player>>,
    main_camera: Res<MainCamera>,
    mut renderer: NonSendMut<Renderer>,
) {
    if let Some(ctx) = renderer.ctx() {
        for (position, remote) in query {
            ctx.gfx
                .text(&remote.name)
                .at(main_camera.cam.world_to_screen(position.as_vec2()))
                .size(PLAYER_NAME_FNT_SIZE);
        }
    }
}

fn update_local_player(
    mut query: Query<&mut Position, With<LocalPlayer>>,
    mut main_camera: ResMut<MainCamera>,
    input: Res<InputCtx>,
    mut renderer: NonSendMut<Renderer>,
) {
    if let Some(ctx) = renderer.ctx() {
        if let Ok(mut position) = query.single_mut() {
            let mut motion: Vec2 = Vec2::ZERO;

            if input.action_held(Action::Down) {
                motion.y += 1.;
            }

            if input.action_held(Action::Up) {
                motion.y -= 1.;
            }

            if input.action_held(Action::Left) {
                motion.x -= 1.;
            }

            if input.action_held(Action::Right) {
                motion.x += 1.;
            }

            let pos: Vec2 = position.as_vec2();

            // Don't use an expensive square root if you don't need it.
            if motion.distance_squared(Vec2::ZERO) != 0. {
                position.from_vec2(pos + motion.normalize() * PLAYER_SPEED * ctx.timer.delta);
            }

            main_camera
                .cam
                .center(position.as_vec2(), ctx.gfx.screen_size());
        }
    }
}
