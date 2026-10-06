use bevy_ecs::prelude::*;
use starbloom_base::prelude::*;
use starbloom_derive::*;

#[derive(Resource, StateSetManager)]
pub struct GameStateManager {
    state: GameState,
}

impl GameStateManager {
    fn new() -> Self {
        Self {
            state: GameState::Bootstrap,
        }
    }
}

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameState {
    Bootstrap,
    Intro,
    Mainloop,
}

pub struct FrameStepPlugin;

impl Plugin for FrameStepPlugin {
    fn create(&self, _world: &mut World, schedule: &mut Schedule) {
        schedule.configure_sets(
            (
                FrameStep::Prepare,
                FrameStep::UpdatePlayer,
                FrameStep::SyncMultiplayer,
                (FrameStep::UpdateMap, FrameStep::UpdateEntities),
                (
                    FrameStep::RenderMap,
                    FrameStep::RenderEntities,
                    FrameStep::RenderUI,
                )
                    .chain(),
                FrameStep::Cleanup,
            )
                .chain()
                .in_set(GameState::Mainloop),
        );
    }
}

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, Clone, Copy)]
pub enum FrameStep {
    Prepare,

    SyncMultiplayer, // Does nothing yet

    UpdatePlayer,
    UpdateMap,
    UpdateEntities,

    RenderMap,
    RenderEntities,
    //Add a RenderPost for post-processing effects when I add them
    RenderUI,

    Cleanup,
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn create(&self, world: &mut World, schedule: &mut Schedule) {
        let manager: GameStateManager = GameStateManager::new();
        manager.configure(schedule, GameState::Bootstrap);
        schedule.configure_sets(
            GameState::Intro.run_if(|state_manager: Res<GameStateManager>| {
                state_manager.state == GameState::Intro
            }),
        );
        schedule.configure_sets(GameState::Mainloop.run_if(
            |state_manager: Res<GameStateManager>| state_manager.state == GameState::Mainloop,
        ));
        world.insert_resource(manager);
        schedule.add_systems(on_state_switch);
    }
}

fn on_state_switch(state_manager: Res<GameStateManager>) {
    if state_manager.is_changed() {
        info!(
            "Switched game state to {:#?} within this frame",
            state_manager.state
        );
    }
}

pub trait StateSetManager<T: SystemSet>: Resource {
    fn configure(&self, schedule: &mut Schedule, state: T) -> &Self;
    fn switch(&mut self, state: T);
}
