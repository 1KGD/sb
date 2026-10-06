use starbloom_base::prelude::*;
use starbloom_bootstrap::*;
use starbloom_camera::*;
use starbloom_input::prelude::*;
use starbloom_states::*;

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

#[derive(Component, Debug, Clone, Copy)]
struct PlayerTexture;

impl Plugin for PlayerPlugin {
    fn create(&self, world: &mut World, schedule: &mut Schedule) {
        declare_texture_asset(world, include_bytes!("../assets/debug.png"), PlayerTexture);
        schedule.add_systems(update_local_player.in_set(FrameStep::UpdatePlayer));
        schedule.add_systems(
            (render_players, render_player_names)
                .chain()
                .in_set(FrameStep::RenderEntities),
        );
        world.spawn(LocalPlayer::default());
    }
}
fn render_players(
    query: Query<&Position, With<Player>>,
    asset: Single<&TextureAsset, With<PlayerTexture>>,
    main_camera: Res<MainCamera>,
    mut renderer: NonSendMut<Renderer>,
) {
    let texture_id: usize = asset.into_inner().id;
    if let Some(ctx) = renderer.ctx() {
        for pos in query {
            ctx.gfx
                .rect()
                .texture(texture_id)
                .anchor(Anchor::Center)
                .size(Vec2::splat(16.))
                .at(main_camera.cam.world_to_screen(**pos));
        }
    }
}

fn render_player_names(
    query: Query<(&Position, &RemotePlayer), With<Player>>,
    main_camera: Res<MainCamera>,
    mut renderer: NonSendMut<Renderer>,
) {
    if let Some(ctx) = renderer.ctx() {
        for (pos, remote) in query {
            ctx.gfx
                .text(&remote.name)
                .at(main_camera.cam.world_to_screen(**pos))
                .size(PLAYER_NAME_FNT_SIZE);
        }
    }
}

fn update_local_player(
    mut query: Query<&mut Position, With<LocalPlayer>>,
    mut main_camera: ResMut<MainCamera>,
    input: Res<InputCtx>,
    frame_data: Res<FrameData>,
) {
    if let Ok(mut pos) = query.single_mut() {
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

        // Don't use an expensive square root if you don't need it.
        if motion.distance_squared(Vec2::ZERO) != 0. {
            **pos = **pos + motion.normalize() * PLAYER_SPEED * frame_data.delta;
        }

        main_camera.cam.center(**pos, frame_data.screen_size);
    }
}
