use starbloom_base::prelude::*;
use starbloom_camera::*;

use crate::chunk::*;

pub struct ChunkloaderPlugin();

impl Plugin for ChunkloaderPlugin {
    fn create(world: &mut World, schedule: &mut Schedule) {
        schedule.add_systems(cull_chunks);
        world.spawn(Chunk::load(0, 0));
    }
}

fn cull_chunks(
    mut commands: Commands,
    query: Query<(Entity, &Chunk)>,
    camera: Res<MainCamera>,
    mut renderer: NonSendMut<Renderer>,
) {
    if let Some(ctx) = renderer.ctx() {
        let viewport: Rect = camera.cam.viewport(ctx.gfx.screen_size());
        for (entity, chunk) in query {
            if chunk.should_be_culled(viewport) {
                commands.entity(entity).despawn();
            }
        }
    }
}
