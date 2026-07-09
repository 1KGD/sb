use bevy_ecs::prelude::*;

use starbloom_base::*;
use starbloom_camera::*;
use starbloom_tiles::*;

mod chunk;
mod chunkloader;

pub use crate::chunk::*;
use crate::chunkloader::*;

pub struct MapPlugin();

impl Plugin for MapPlugin {
    fn create(world: &mut World, schedule: &mut Schedule) {
        ChunkloaderPlugin::create(world, schedule);

        world.insert_resource(TileRegestry::new());
        schedule.add_systems(render_chunks.after(prepare_camera));
    }
}

pub fn render_chunks(query: Query<&Chunk>, tile_regestry: Res<TileRegestry>) {
    for chunk in &query {
        chunk.render(&tile_regestry);
    }
}
