use crate::*;

pub struct MapServerPlugin;

impl Plugin<ServerPlugin> for MapServerPlugin {
    fn create(&self, _world: &mut World, schedule: &mut Schedule) {
        schedule.add_systems(spawn_requested_chunks.in_set(ServerStep::UpdateMap));
    }
}

fn spawn_requested_chunks(mut commands: Commands, mut requests: MessageReader<ChunkRequest>) {
    for request in requests.read() {
        debug!("Spawned chunk at {}", request.0);
        commands.spawn(Chunk {
            pos: request.0,
            tiles: Some([[2; CHUNK_DIM]; CHUNK_DIM]),
        });
    }
}
