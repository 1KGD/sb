use starbloom_base::*;

use crate::input_ctx::*;

mod actions;
mod input_ctx;
pub mod prelude;

pub struct InputPlugin;

impl Plugin<ClientPlugin> for InputPlugin {
    fn create(
        &self,
        world: &mut bevy_ecs::world::World,
        _schedule: &mut bevy_ecs::schedule::Schedule,
    ) {
        world.insert_resource(InputCtx::default());
    }
}
