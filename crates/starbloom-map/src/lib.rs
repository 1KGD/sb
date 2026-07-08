use bevy_ecs::prelude::*;

use starbloom_base::*;

mod chunk;
mod chunkloader;
mod tile;

pub use crate::chunk::*;
use crate::chunkloader::*;
pub use crate::tile::*;

pub struct MapPlugin();

impl Plugin for MapPlugin {
    fn create(world: &mut World, schedule: &mut Schedule) {
        ChunkloaderPlugin::create(world, schedule);

        world.insert_resource(TileRegestry::new());
        schedule.add_systems(render_chunks);
    }
}

fn render_chunks(query: Query<&Chunk>, tile_regestry: Res<TileRegestry>) {
    for chunk in &query {
        chunk.render(&tile_regestry);
    }
}
