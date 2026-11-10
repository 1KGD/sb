#[sided(Client)]
use starbloom_core_client::prelude::*;
use starbloom_core_shared::prelude::*;

#[sided(Client)]
use crate::presents_sequence::*;
#[sided(Client)]
use crate::rust_sequence::*;
#[sided(Client)]
use crate::title_sequence::*;

pub struct IntroPlugin;

mod presents_sequence;
mod rust_sequence;
mod title_sequence;

#[sided(Client)]
impl Plugin<ClientPlugin> for IntroPlugin {
    fn create(&self, world: &mut World, schedule: &mut Schedule) {
        let manager: IntroStateManager = IntroStateManager::new();
        schedule.configure_sets(
            (IntroState::Rust, IntroState::Title, IntroState::Finish).in_set(ClientState::Intro),
        );
        manager
            .configure(schedule, IntroState::Rust)
            .configure(schedule, IntroState::Presents)
            .configure(schedule, IntroState::Title)
            .configure(schedule, IntroState::Finish);
        world.insert_resource(manager);

        schedule.add_systems(catch_finish.in_set(IntroState::Finish));

        schedule.add_systems(clear_background);

        RustSequencePlugin.create(world, schedule);
        PresentsSequencePlugin.create(world, schedule);
        TitleSequencePlugin.create(world, schedule);
    }
}

#[sided(Client)]
#[derive(Resource, StateSetManager)]
struct IntroStateManager {
    state: IntroState,
}

#[sided(Client)]
impl IntroStateManager {
    fn new() -> Self {
        Self {
            state: IntroState::Rust,
        }
    }
}

#[sided(Client)]
#[derive(SystemSet, Hash, Debug, PartialEq, Eq, Clone, Copy)]
enum IntroState {
    Rust,
    Presents,
    Title,
    Finish,
}

#[sided(Client)]
fn catch_finish(mut game_manager: ResMut<ClientStateManager>) {
    game_manager.switch(ClientState::MainMenu);
}

#[sided(Client)]
pub(crate) fn clear_background(mut renderer: NonSendMut<Renderer>) {
    renderer.clear(Color::BLACK);
}
