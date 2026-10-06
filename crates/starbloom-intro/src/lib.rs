use starbloom_base::prelude::*;
use starbloom_derive::*;
use starbloom_states::*;

use crate::title_sequence::*;

pub struct IntroPlugin;

mod title_sequence;

const INTRO_PHASE_TIME: f32 = 5.;

impl Plugin for IntroPlugin {
    fn create(&self, world: &mut World, schedule: &mut Schedule) {
        let manager: IntroStateManager = IntroStateManager::new();
        schedule.configure_sets(
            (IntroState::Rust, IntroState::Title, IntroState::Finish).in_set(GameState::Intro),
        );
        manager
            .configure(schedule, IntroState::Rust)
            .configure(schedule, IntroState::Title)
            .configure(schedule, IntroState::Finish);
        world.insert_resource(manager);

        TitleSequencePlugin.create(world, schedule);
    }
}

#[derive(Resource, StateSetManager)]
struct IntroStateManager {
    state: IntroState,
}

impl IntroStateManager {
    fn new() -> Self {
        Self {
            state: IntroState::Title,
        }
    }
}

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, Clone, Copy)]
enum IntroState {
    Rust,
    Presents,
    Title,
    Finish,
}
