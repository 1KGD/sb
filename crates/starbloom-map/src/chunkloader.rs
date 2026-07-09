use bevy_ecs::prelude::*;
use macroquad::prelude::*;

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
    let bounds: Rect = Rect::new(
        camera.position.x - screen_width() / 2.,
        camera.position.y - screen_height() / 2.,
        screen_width(),
        screen_height(),
    );

    for (entity, chunk) in query {
        if !bounds.overlaps(&chunk.get_bounding_rect()) {
            commands.entity(entity).despawn();
        }
    }
}
