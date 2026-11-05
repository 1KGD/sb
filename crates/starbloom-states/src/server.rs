use log::*;
use starbloom_base::*;
use starbloom_derive::*;

use crate::*;

#[derive(Resource, StateSetManager)]
pub struct ServerStateManager {
    state: ServerState,
}

impl ServerStateManager {
    fn new() -> Self {
        Self {
            state: ServerState::Mainloop,
        }
    }
}

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, Clone, Copy)]
pub enum ServerState {
    // TODO: Add world load state
    Mainloop,
}

pub struct ServerStepPlugin;

impl Plugin<ServerPlugin> for ServerStepPlugin {
    fn create(&self, _world: &mut World, schedule: &mut Schedule) {
        schedule.configure_sets(
            (
                ServerStep::ReceivePackets,
                ServerStep::ProcessConnections,
                (
                    ServerStep::UpdatePlayers,
                    ServerStep::UpdateMap,
                    ServerStep::UpdateEntities,
                ),
                ServerStep::SendPackets,
            )
                .chain()
                .in_set(ServerState::Mainloop),
        );
    }
}

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, Clone, Copy)]
pub enum ServerStep {
    ReceivePackets,

    ProcessConnections,

    UpdatePlayers,
    UpdateMap,
    UpdateEntities,

    SendPackets,
}

pub struct ServerStatePlugin;

impl Plugin<ServerPlugin> for ServerStatePlugin {
    fn create(&self, world: &mut World, schedule: &mut Schedule) {
        let manager: ServerStateManager = ServerStateManager::new();
        manager.configure(schedule, ServerState::Mainloop);
        schedule.configure_sets(ServerState::Mainloop.run_if(
            |state_manager: Res<ServerStateManager>| state_manager.state == ServerState::Mainloop,
        ));
        world.insert_resource(manager);
        schedule.add_systems(on_state_switch);
    }
}

fn on_state_switch(state_manager: Res<ServerStateManager>) {
    if state_manager.is_changed() {
        info!(
            "Switched SERVER state to {:#?} within this frame",
            state_manager.state
        );
    }
}
