use crate::*;

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn create(&self, world: &mut World, schedule: &mut Schedule) {
        world.insert_resource(GameStateManager::new());
        schedule.configure_sets(GameState::Bootstrap.run_if(
            |state_manager: Res<GameStateManager>| state_manager.state == GameState::Bootstrap,
        ));
        schedule.configure_sets(GameState::Mainloop.run_if(
            |state_manager: Res<GameStateManager>| state_manager.state == GameState::Mainloop,
        ));
        schedule.add_systems(on_state_switch);
    }
}

pub(crate) fn finish_bootstrap(mut state_manager: ResMut<GameStateManager>) {
    state_manager.state = GameState::Mainloop;
}

fn on_state_switch(state_manager: Res<GameStateManager>) {
    if state_manager.is_changed() {
        info!(
            "Switched game state to {:#?} within this frame",
            state_manager.state
        );
    }
}

#[derive(Resource)]
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
