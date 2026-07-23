use bevy_ecs::prelude::*;
use egor::math::*;

use starbloom_base::prelude::*;

pub struct CameraPlugin();

impl Plugin for CameraPlugin {
    fn create(world: &mut World, _schedule: &mut Schedule) {
        world.insert_resource(MainCamera::default());
    }
}

#[derive(Resource, Default)]
pub struct MainCamera {
    pub position: Vec2,
}
