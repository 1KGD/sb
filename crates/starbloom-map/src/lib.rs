use bevy_ecs::prelude::*;

use starbloom_base::prelude::*;
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
        schedule.add_systems(render_chunks);
        world.insert_resource(TileRegestry::new());
    }
}

pub fn render_chunks(
    query: Query<&Chunk>,
    tile_regestry: Res<TileRegestry>,
    main_camera: Res<MainCamera>,
    mut gfx: NonSendMut<GfxCmds>,
) {
    for chunk in &query {
        chunk.render(&mut gfx, &main_camera, &tile_regestry);
    }
}
