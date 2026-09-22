use starbloom_base::prelude::*;

pub const TILE_SIZE: f32 = 8.;

pub type TileRepr = u16;

mod grass;
mod plugin;

pub use crate::plugin::*;

pub struct Tile {
    pub entity: Entity,
    pub texture: Option<usize>,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct TileComponent;

impl Tile {
    pub fn declare<T: Component + std::fmt::Debug + Copy>(
        world: &mut World,
        texture_data: Option<&'static [u8]>,
        bundle: T,
    ) -> Self {
        Self {
            entity: texture_data.map_or(
                world.spawn((bundle, TileComponent)).id(),
                |texture: &[u8]| declare_texture_asset(world, texture, (bundle, TileComponent)),
            ),
            texture: None,
        }
    }
}

#[derive(Resource)]
pub struct TileRegestry {
    ids: std::collections::BTreeMap<TileRepr, Entity>,
    entries: std::collections::HashMap<Entity, Tile>,
}

impl TileRegestry {
    pub fn new() -> Self {
        Self {
            ids: std::collections::BTreeMap::new(),
            entries: std::collections::HashMap::new(),
        }
    }
    pub fn regester(&mut self, tile: Tile) {
        let entity: Entity = tile.entity;
        self.entries.insert(entity, tile);
        if self.ids.values().find(|v| **v == entity).is_none() {
            self.ids.insert(self.ids.len() as TileRepr, entity);
        }
        debug!("Regestered tile '{}'", entity);
    }

    pub fn get_tile_by_idx(&self, idx: &TileRepr) -> &Tile {
        self.entries
            .get(self.ids.get(idx).expect("bad tile idx"))
            .expect("Bad tile idx")
    }
}
