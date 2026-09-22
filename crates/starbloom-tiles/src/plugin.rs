use crate::grass::*;
use crate::*;

pub struct DefaultTilePlugin;

impl Plugin for DefaultTilePlugin {
    fn create(world: &mut World, schedule: &mut Schedule) {
        schedule.add_systems(insert_tile_textures_into_regestry.after(load_assets));
        regester_grass(world);
    }
}

fn insert_tile_textures_into_regestry(
    query: Query<(Entity, &TextureAsset), With<TileComponent>>,
    mut regestry: ResMut<TileRegestry>,
) {
    for (entity, asset) in query {
        if let Some(entry) = regestry.entries.get_mut(&entity) {
            entry.texture = Some(asset.id);
        }
    }
}
