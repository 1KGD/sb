use crate::*;

pub struct MapClientPlugin;

impl Plugin<ClientPlugin> for MapClientPlugin {
    fn create(&self, world: &mut World, schedule: &mut Schedule) {
        schedule.add_systems((request_chunks, cull_chunks).in_set(ClientStep::UpdateMap));
        schedule.add_systems(render_chunks.in_set(ClientStep::RenderMap));
        world.insert_resource(Messages::<ChunkRequest>::default());
    }
}

fn request_chunks(mut requests: MessageWriter<ChunkRequest>, mut triggered: Local<bool>) {
    if !*triggered {
        requests.write(ChunkRequest(ivec2(1, 0)));
        requests.write(ChunkRequest(ivec2(-1, 0)));
        *triggered = true;
    }
}

fn cull_chunks(
    mut commands: Commands,
    chunks: Query<(Entity, &Chunk)>,
    frame_data: Res<FrameData>,
    main_camera: Res<MainCamera>,
) {
    let viewport: Rect = main_camera.viewport(frame_data.screen_size);
    for (entity, chunk) in chunks {
        if chunk.should_be_culled(viewport) {
            commands.entity(entity).despawn();
        }
    }
}

fn render_chunks(
    query: Query<&Chunk>,
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
                        let pos: Vec2 = main_camera.world_to_screen(
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
