#[sided(Client)]
use crate::grass::*;
use crate::*;

pub struct DefaultTilePlugin;

#[sided(Client)]
impl Plugin<SharedPlugin> for DefaultTilePlugin {
    fn create(&self, world: &mut World, schedule: &mut Schedule) {
        world.insert_resource(TileRegestry::new());
        schedule.add_systems(
            insert_tile_textures_into_regestry
                .after(load_assets)
                .in_set(ClientState::Bootstrap),
        );
        regester_grass(world);
    }
}

#[sided(Client)]
fn insert_tile_textures_into_regestry(
    query: Query<(Entity, &TextureAsset), With<TileComponent>>,
    mut regestry: ResMut<TileRegestry>,
) {
    for (entity, asset) in query {
        if let Some(entry) = regestry.entries.get_mut(&entity) {
            entry.texture = Some(**asset);
        }
    }
}
