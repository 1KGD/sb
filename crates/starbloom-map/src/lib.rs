use starbloom_base::prelude::*;
use starbloom_camera::*;
use starbloom_tiles::*;

mod chunk;
mod chunkloader;
mod data;

pub use crate::chunk::*;
use crate::chunkloader::*;
use crate::data::*;

pub struct MapPlugin();

impl Plugin for MapPlugin {
    fn create(world: &mut World, schedule: &mut Schedule) {
        ChunkloaderPlugin::create(world, schedule);
        schedule.add_systems(generate_chunks);
        schedule.add_systems(render_chunks.after(generate_chunks));
        world.insert_resource(TileRegestry::new());
        world.insert_resource(ChunkDataProvider {});
    }
}

pub fn generate_chunks(mut query: Query<&mut Chunk>, data_provider: Res<ChunkDataProvider>) {
    for mut chunk in &mut query {
        chunk.generate(&data_provider);
    }
}

pub fn render_chunks(
    query: Query<&Chunk>,
    tile_regestry: Res<TileRegestry>,
    main_camera: Res<MainCamera>,
    mut renderer: NonSendMut<Renderer>,
) {
    for chunk in &query {
        chunk.render(&mut renderer, &main_camera, &tile_regestry);
    }
}
