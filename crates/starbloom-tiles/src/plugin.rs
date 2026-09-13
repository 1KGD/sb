use starbloom_base::prelude::*;

use crate::{TileRegestry, grass::*};

pub struct DefaultTilePlugin;

impl Plugin for DefaultTilePlugin {
    fn create(world: &mut World, _schedule: &mut Schedule) {
        let mut regestry: Mut<'_, TileRegestry> = world.get_resource_mut::<TileRegestry>().unwrap();
        regester_grass(&mut regestry);
    }
}
