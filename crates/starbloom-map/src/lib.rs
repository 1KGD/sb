#[sided(Client)]
use starbloom_core_client::prelude::*;
use starbloom_core_shared::prelude::*;
#[sided(Client)]
use starbloom_tiles::*;

mod chunk;
#[sided(Client)]
mod client;

pub use crate::chunk::*;
#[sided(Client)]
pub use crate::client::*;

pub struct MapClientPlugin;

#[sided(Client)]
impl Plugin<ClientPlugin> for MapClientPlugin {
    fn create(&self, world: &mut World, schedule: &mut Schedule) {
        schedule.add_systems(render_chunks.in_set(ClientStep::RenderMap));
        world.spawn((
            ClientChunk {},
            Chunk {
                pos: IVec2::ZERO,
                tiles: Some([[1; CHUNK_DIM]; CHUNK_DIM]),
            },
        ));
    }
}

#[sided(Client)]
pub fn render_chunks(
    query: Query<&Chunk, With<ClientChunk>>,
    tile_regestry: Res<TileRegestry>,
    main_camera: Res<MainCamera>,
    mut renderer: NonSendMut<Renderer>,
) {
    for chunk in &query {
        if let Some(tiles) = chunk.tiles {
            for (x, row) in tiles.iter().enumerate() {
                for (y, tile_idx) in row.iter().enumerate() {
                    let tile = tile_regestry.get_tile_by_idx(tile_idx);
                    if let Some(texture) = tile.texture {
                        let pos: Vec2 = main_camera.cam.world_to_screen(
                            vec2(x as f32, y as f32) * TILE_SIZE + chunk.get_world_pos(),
                        );
                        renderer
                            .rect()
                            .texture(texture)
                            .size(vec2(TILE_SIZE, TILE_SIZE))
                            .at(pos);
                    }
                }
            }
        }
    }
}
