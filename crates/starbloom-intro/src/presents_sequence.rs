use crate::*;

const SEQUENCE_TIME: f32 = 7.;

const TEXT: &'static str = include_str!("../../../Cargo.toml"); // Rule of cool

pub(crate) struct PresentsSequencePlugin;

impl Plugin<ClientPlugin> for PresentsSequencePlugin {
    fn create(&self, _world: &mut World, schedule: &mut Schedule) {
        schedule.add_systems(all_the_text_stuff_in_one_system.in_set(IntroState::Presents));
    }
}

fn all_the_text_stuff_in_one_system(
    mut renderer: NonSendMut<Renderer>,
    mut progress: Local<f32>,
    mut manager: ResMut<IntroStateManager>,
    frame_data: Res<FrameData>,
) {
    *progress += frame_data.delta / SEQUENCE_TIME;
    if let Some(text) = TEXT
        .chars()
        .collect::<Vec<char>>()
        .get(
            ((*progress * TEXT.chars().count() as f32) - 1000.).max(0.) as usize
                ..(*progress * TEXT.chars().count() as f32) as usize,
        )
        .map(|t: &[char]| t.iter().collect::<String>())
    {
        renderer
            .gfx
            .text(&text)
            .color(Color::GREEN)
            .size(3.)
            .font("Pixelify Sans".to_owned());

        if *progress > 0.3 {
            renderer
                .gfx
                .text("John Schiefelbein")
                .color(Color::WHITE)
                .font("Pixelify Sans".to_owned())
                .in_rect(
                    Rect::new(
                        Vec2::ZERO,
                        vec2(frame_data.screen_size.x, frame_data.screen_size.y / 2.),
                    ),
                    Align::BottomCenter,
                );
        }

        if *progress > 0.6 {
            renderer
                .gfx
                .text("Presents")
                .color(Color::WHITE)
                .font("Pixelify Sans".to_owned())
                .in_rect(
                    Rect::new(
                        vec2(0., frame_data.screen_size.y / 2.),
                        vec2(frame_data.screen_size.x, frame_data.screen_size.y / 2.),
                    ),
                    Align::TopCenter,
                );
        }
    } else {
        manager.switch(IntroState::Title);
    }
}
