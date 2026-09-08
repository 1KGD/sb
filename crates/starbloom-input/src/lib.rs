use starbloom_base::prelude::*;

use crate::input_ctx::*;

mod input_ctx;
mod actions;
pub mod prelude;

pub struct InputPlugin();

impl Plugin for InputPlugin {
    fn create(world: &mut bevy_ecs::world::World, _schedule: &mut bevy_ecs::schedule::Schedule) {
        world.insert_resource(InputCtx::default());
    }
}
