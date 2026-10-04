use starbloom_base::prelude::*;

pub struct IntroPlugin;

const INTRO_PHASE_TIME: f32 = 5.;

impl Plugin for IntroPlugin {
    fn create(&self, world: &mut World, schedule: &mut Schedule) {
        world.insert_resource(IntroStateManager::new());
    }
}

#[derive(Resource)]
struct IntroStateManager(IntroState);

impl IntroStateManager {
    fn new() -> Self {
        Self(IntroState::Rust)
    }
}

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, Clone, Copy)]
enum IntroState {
    Rust,
    Presents,
    Title,
    Finish,
}
