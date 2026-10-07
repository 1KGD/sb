use starbloom_base::prelude::*;
use starbloom_bootstrap::TextureAsset;
use starbloom_derive::*;
use starbloom_states::*;

use crate::rust_sequence::*;
use crate::title_sequence::*;
use crate::presents_sequence::*;

pub struct IntroPlugin;

mod presents_sequence;
mod rust_sequence;
mod title_sequence;

impl Plugin for IntroPlugin {
    fn create(&self, world: &mut World, schedule: &mut Schedule) {
        let manager: IntroStateManager = IntroStateManager::new();
        schedule.configure_sets(
            (IntroState::Rust, IntroState::Title, IntroState::Finish).in_set(GameState::Intro),
        );
        manager
            .configure(schedule, IntroState::Rust)
            .configure(schedule, IntroState::Presents)
            .configure(schedule, IntroState::Title)
            .configure(schedule, IntroState::Finish);
        world.insert_resource(manager);

        schedule.add_systems(catch_finish.in_set(IntroState::Finish));

        RustSequencePlugin.create(world, schedule);
        PresentsSequencePlugin.create(world, schedule);
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
            state: IntroState::Rust,
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

fn catch_finish(mut game_manager: ResMut<GameStateManager>) {
    game_manager.switch(GameState::Mainloop);
}
