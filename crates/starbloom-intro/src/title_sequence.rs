#[sided(Client)]
use core::f32;
#[sided(Client)]
use rand::prelude::*;
#[sided(Client)]
use std::ops::{Deref, DerefMut};

use crate::*;

#[sided(Client)]
const SEQUENCE_TIME: f32 = 15.;

#[sided(Client)]
pub(crate) struct TitleSequencePlugin;

#[sided(Client)]
impl Plugin<ClientPlugin> for TitleSequencePlugin {
    fn create(&self, _world: &mut World, schedule: &mut Schedule) {
        schedule.add_systems(
            (
                spawn_starfall,
                update_starfall,
                (render_starfall, render_title).chain(),
            )
                .in_set(IntroState::Title),
        );
    }
}

#[sided(Client)]
#[derive(Component)]
#[require(Position)]
struct Starfall {
    length: f32,
    speed: f32,
}

#[sided(Client)]
struct LocalRng(StdRng);

#[sided(Client)]
impl Default for LocalRng {
    fn default() -> Self {
        Self(StdRng::seed_from_u64(0))
    }
}

#[sided(Client)]
impl Deref for LocalRng {
    type Target = StdRng;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[sided(Client)]
impl DerefMut for LocalRng {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[sided(Client)]
fn render_title(
    mut renderer: NonSendMut<Renderer>,
    frame_data: Res<FrameData>,
    mut manager: ResMut<IntroStateManager>,
    mut age: Local<f32>,
) {
    let screen_rect: Rect = Rect::new(Vec2::ZERO, renderer.gfx.screen_size());
    renderer
        .gfx
        .text("STARBLOOM")
        .font("Pixelify Sans".to_owned())
        .color(Color::new([
            1.,
            1.,
            1.,
            1. - (*age * f32::consts::PI).cos().powi(2),
        ]))
        .in_rect(screen_rect, Align::MiddleCenter);
    *age += frame_data.delta / SEQUENCE_TIME;

    if *age > 1. {
        manager.switch(IntroState::Finish);
    }
}

#[sided(Client)]
fn spawn_starfall(
    mut commands: Commands,
    mut timer: Local<f32>,
    mut rng: Local<LocalRng>,
    frame_data: Res<FrameData>,
) {
    if *timer >= 0.2 {
        let length: f32 = (rng.next_u32() as f32 / u32::MAX as f32) * 20. + 20.;
        let speed: f32 = ((rng.next_u32() as f32 / u32::MAX as f32) * 20. + 20.).floor();
        let origin: Vec2 = vec2(
            ((rng.next_u32() as f32 / u32::MAX as f32)
                * (frame_data.screen_size.x + frame_data.screen_size.y))
                .floor(),
            0.,
        );
        commands.spawn((Starfall { length, speed }, Position::from(origin)));
        *timer = 0.;
    }
    *timer += frame_data.delta;
}

#[sided(Client)]
fn render_starfall(query: Query<(&Position, &Starfall)>, mut renderer: NonSendMut<Renderer>) {
    for (pos, starfall) in query {
        renderer
            .gfx
            .polyline()
            .points(&[**pos, **pos - vec2(-1., 1.) * starfall.length])
            .color(Color::WHITE);
    }
}

#[sided(Client)]
fn update_starfall(
    mut commands: Commands,
    query: Query<(Entity, &mut Position, &Starfall)>,
    frame_data: Res<FrameData>,
) {
    for (entity, mut pos, starfall) in query {
        **pos += vec2(-1., 1.) * starfall.speed * frame_data.delta;
        if pos.x < -starfall.length {
            commands.entity(entity).despawn();
        }
    }
}
