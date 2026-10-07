use starbloom_bootstrap::*;

use crate::*;

const SEQUENCE_TIME: f32 = 5.;

#[derive(Component, Clone, Copy, Debug)]
struct FerrisTexture;

static FERRIS_TEXTURE_SIZE: std::sync::LazyLock<Vec2> =
    std::sync::LazyLock::new(|| vec2(420., 307.) / 5.);

pub struct RustSequencePlugin;

impl Plugin for RustSequencePlugin {
    fn create(&self, world: &mut World, schedule: &mut Schedule) {
        declare_texture_asset(world, include_bytes!("../assets/ferris.png"), FerrisTexture);
        schedule.add_systems((render_ferris, rust_sequence_timeout).in_set(IntroState::Rust));
    }
}

fn render_ferris(
    texture: Single<&TextureAsset, With<FerrisTexture>>,
    mut renderer: NonSendMut<Renderer>,
) {
    if let Some(ctx) = renderer.ctx() {
        let screen_size: Vec2 = ctx.gfx.screen_size();
        ctx.gfx
            .rect()
            .texture(texture.id)
            .size(*FERRIS_TEXTURE_SIZE)
            .at(screen_size / 2. - *FERRIS_TEXTURE_SIZE / 2.);

        ctx.gfx
            .text("Made with Rust")
            .font("Pixelify Sans".to_owned())
            .color(Color::WHITE)
            .in_rect(
                Rect::new(
                    vec2(0., screen_size.y / 2. + FERRIS_TEXTURE_SIZE.y / 2. + 10.),
                    vec2(screen_size.x, screen_size.y / 2.),
                ),
                Align::TopCenter,
            );
    }
}

fn rust_sequence_timeout(
    mut timer: Local<f32>,
    mut manager: ResMut<IntroStateManager>,
    frame_data: Res<FrameData>,
) {
    *timer += frame_data.delta;
    if *timer >= SEQUENCE_TIME {
        manager.switch(IntroState::Presents);
    }
}
