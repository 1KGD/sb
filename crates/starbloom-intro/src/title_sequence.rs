use rand::prelude::*;
use std::ops::{Deref, DerefMut};

use crate::*;

pub(crate) struct TitleSequencePlugin;

impl Plugin for TitleSequencePlugin {
    fn create(&self, _world: &mut World, schedule: &mut Schedule) {
        schedule.add_systems(
            (spawn_starfall, update_starfall, render_starfall).in_set(IntroState::Title),
        );
    }
}

#[derive(Component)]
#[require(Position)]
struct Starfall {
    length: f32,
    speed: f32,
}

struct LocalRng(StdRng);

impl Default for LocalRng {
    fn default() -> Self {
        Self(StdRng::seed_from_u64(0))
    }
}

impl Deref for LocalRng {
    type Target = StdRng;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LocalRng {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn spawn_starfall(
    mut commands: Commands,
    mut timer: Local<f32>,
    mut rng: Local<LocalRng>,
    frame_data: Res<FrameData>,
) {
    if *timer >= 0.2 {
        let length: f32 = (rng.next_u32() as f32 / u32::MAX as f32) * 20. + 20.;
        let speed: f32 = (rng.next_u32() as f32 / u32::MAX as f32) * 20. + 20.;
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

fn render_starfall(query: Query<(&Position, &Starfall)>, mut renderer: NonSendMut<Renderer>) {
    if let Some(ctx) = renderer.ctx() {
        for (pos, starfall) in query {
            ctx.gfx
                .polyline()
                .points(&[**pos, **pos - vec2(-1., 1.) * starfall.length])
                .color(Color::WHITE);
        }
    }
}

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
