use bevy_ecs::prelude::*;

use starbloom_base::*;
use starbloom_camera::*;

use crate::chunk::*;

pub struct ChunkloaderPlugin();

impl Plugin for ChunkloaderPlugin {
    fn create(world: &mut World, schedule: &mut Schedule) {
        schedule.add_systems(chunkloading_task);
        world.spawn(Chunk::load(0, 0));
    }
}

fn chunkloading_task(
    mut commands: Commands,
    query: Query<(Entity, &Chunk)>,
    camera: Res<MainCamera>,
) {
}
