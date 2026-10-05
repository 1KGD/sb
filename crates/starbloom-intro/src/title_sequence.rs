use std::ops::Div;

use crate::*;

pub(crate) struct TitleSequencePlugin;

impl Plugin for TitleSequencePlugin {
    fn create(&self, _world: &mut World, schedule: &mut Schedule) {
        schedule.add_systems((spawn_starfall, render_starfall).in_set(IntroState::Title));
    }
}

#[derive(Component)]
#[require(Position)]
struct Starfall;

fn spawn_starfall(
    mut commands: Commands,
    mut timer: Local<f32>,
    mut renderer: NonSendMut<Renderer>,
) {
    if let Some(ctx) = renderer.ctx() {
        if *timer >= 0.2 {
            commands.spawn((Starfall, Position::default()));
            *timer = 0.;
        }
        *timer += ctx.timer.delta;
    }
}

fn render_starfall(query: Query<&Position, With<Starfall>>, mut renderer: NonSendMut<Renderer>) {
    if let Some(ctx) = renderer.ctx() {
        for pos in query {
            ctx.gfx.path().circle(5.).fill_color(Color::WHITE).at(**pos);
        }
    }
}
