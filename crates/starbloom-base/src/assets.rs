use std::collections::HashMap;

use bevy_ecs::prelude::*;
use egor::app::*;
use log::*;

#[derive(Resource, Default)]
pub struct AssetServer {
    textures: HashMap<&'static str, usize>,
}

impl AssetServer {
    pub fn bind_texture(
        &mut self,
        ctx: &mut Box<&mut &mut FrameContext>,
        name: &'static str,
        data: &[u8],
    ) -> usize {
        if let Some(id) = self.textures.get(name).map(|id| *id) {
            return id;
        };
        let id: usize = ctx.gfx.load_texture(data);
        debug!("Loaded texture {} with id {}", name, id);
        self.textures.insert(name, id);
        id
    }
}
