use log::*;
use starbloom_base::*;
use starbloom_derive::*;

use crate::*;

#[derive(Resource, StateSetManager)]
pub struct ClientStateManager {
    state: ClientState,
}

impl ClientStateManager {
    fn new() -> Self {
        Self {
            state: ClientState::Bootstrap,
        }
    }
}

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, Clone, Copy)]
pub enum ClientState {
    Bootstrap,
    Intro,
    MainMenu,
    Mainloop,
}

pub struct ClientStepPlugin;

impl Plugin<ClientPlugin> for ClientStepPlugin {
    fn create(&self, _world: &mut World, schedule: &mut Schedule) {
        schedule.configure_sets(
            (
                ClientStep::Prepare,
                ClientStep::UpdatePlayer,
                ClientStep::SyncMultiplayer,
                (ClientStep::UpdateMap, ClientStep::UpdateEntities),
                (
                    ClientStep::RenderMap,
                    ClientStep::RenderEntities,
                    ClientStep::RenderUI,
                )
                    .chain(),
                ClientStep::Cleanup,
            )
                .chain()
                .in_set(ClientState::Mainloop),
        );
    }
}

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, Clone, Copy)]
pub enum ClientStep {
    Prepare,

    SyncMultiplayer, // Does nothing yet

    RenderMap,
    RenderEntities,
    //Add a RenderPost for post-processing effects (e.g. lighting) when I add them
    RenderUI,

    Cleanup,
}

pub struct ClientStatePlugin;

impl Plugin<ClientPlugin> for ClientStatePlugin {
    fn create(&self, world: &mut World, schedule: &mut Schedule) {
        let manager: ClientStateManager = ClientStateManager::new();
        manager.configure(schedule, ClientState::Bootstrap);
        schedule.configure_sets(ClientState::Intro.run_if(
            |state_manager: Res<ClientStateManager>| state_manager.state == ClientState::Intro,
        ));
        schedule.configure_sets(ClientState::Mainloop.run_if(
            |state_manager: Res<ClientStateManager>| state_manager.state == ClientState::Mainloop,
        ));
        world.insert_resource(manager);
        schedule.add_systems(on_state_switch);
    }
}

fn on_state_switch(state_manager: Res<ClientStateManager>) {
    if state_manager.is_changed() {
        info!(
            "Switched CLIENT state to {:#?} within this frame",
            state_manager.state
        );
    }
}
