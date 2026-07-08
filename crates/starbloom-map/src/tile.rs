use bevy_ecs::prelude::*;
use macroquad::prelude::*;

pub type TileRepr = u16;

pub struct Tile {
    pub renderable: bool,
}

impl Tile {
    pub const fn declare(renderable: bool) -> Self {
        Self { renderable }
    }
}

#[derive(Resource)]
pub struct TileRegestry {
    ids: std::collections::BTreeMap<TileRepr, &'static str>,
    entries: std::collections::HashMap<&'static str, Tile>,
}

impl TileRegestry {
    pub fn new() -> Self {
        Self {
            ids: std::collections::BTreeMap::new(),
            entries: std::collections::HashMap::new(),
        }
    }

    pub fn regester(&mut self, id: &'static str, tile: Tile) {
        self.entries.insert(id, tile);
        if self.ids.values().find(|v| **v == id).is_none() {
            self.ids.insert(self.ids.len() as TileRepr, id);
        }
        info!("Regestered tile '{}'", id);
    }

    pub fn get_tile_by_idx(&self, idx: &TileRepr) -> &Tile {
        self.entries
            .get(self.ids.get(idx).expect("bad tile idx"))
            .expect("Bad tile idx")
    }
}
