use std::collections::HashMap;

use starbloom_base::prelude::*;
use starbloom_states::*;
use starbloom_camera::*;

use crate::{chunk::*, generate_chunks};

pub struct ChunkloaderPlugin;

impl Plugin for ChunkloaderPlugin {
    fn create(&self, world: &mut World, schedule: &mut Schedule) {
        schedule.add_systems(
            (cull_chunks, load_chunks)
                .before(generate_chunks)
                .in_set(FrameStep::UpdateMap),
        );
        world.insert_resource(ChunkManager::new());
    }
}

#[derive(Resource)]
pub struct ChunkManager {
    chunks: HashMap<IVec2, Entity>,
}

impl ChunkManager {
    fn new() -> Self {
        Self {
            chunks: HashMap::new(),
        }
    }

    fn has_chunk(&self, pos: &IVec2) -> bool {
        self.chunks.contains_key(&pos)
    }

    fn spawn_chunk(&mut self, commands: &mut Commands, pos: IVec2, viewport: Rect) {
        let chunk_component = Chunk::load(pos);
        if chunk_component.should_be_culled(viewport) {
            return;
        }
        let chunk: Entity = commands.spawn(chunk_component).id();
        self.add_chunk(pos, chunk);
    }

    fn add_chunk(&mut self, pos: IVec2, chunk: Entity) {
        if self.has_chunk(&pos) {
            panic!("Chunk already in manager at pos {}", pos);
        }
        self.chunks.insert(pos, chunk);
    }

    fn remove_chunk(&mut self, pos: IVec2) {
        self.chunks.remove(&pos);
    }

    fn spawn_new_chunks(&mut self, mut commands: Commands, viewport: Rect) {
        let min: IVec2 = (viewport.min() / CHUNK_SIZE).as_ivec2() - IVec2::ONE;
        let max: IVec2 = (viewport.max() / CHUNK_SIZE).as_ivec2() + IVec2::ONE;

        for x in min.x..max.x {
            for y in min.y..max.y {
                let pos: IVec2 = ivec2(x, y);
                if !self.has_chunk(&pos) {
                    self.spawn_chunk(&mut commands, pos, viewport);
                }
            }
        }
    }
}

pub fn load_chunks(
    commands: Commands,
    camera: Res<MainCamera>,
    mut manager: ResMut<ChunkManager>,
    mut renderer: NonSendMut<Renderer>,
) {
    if let Some(ctx) = renderer.ctx() {
        let viewport: Rect = camera.cam.viewport(ctx.gfx.screen_size());
        manager.spawn_new_chunks(commands, viewport);
    }
}

pub fn cull_chunks(
    mut commands: Commands,
    query: Query<(Entity, &Chunk)>,
    camera: Res<MainCamera>,
    mut manager: ResMut<ChunkManager>,
    mut renderer: NonSendMut<Renderer>,
) {
    if let Some(ctx) = renderer.ctx() {
        let viewport: Rect = camera.cam.viewport(ctx.gfx.screen_size());
        for (entity, chunk) in query {
            if chunk.should_be_culled(viewport) {
                manager.remove_chunk(chunk.pos);
                commands.entity(entity).despawn();
            }
        }
    }
}
