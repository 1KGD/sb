use bevy_ecs::prelude::*;
use egor_glue::camera::*;
use starbloom_base::*;

pub struct CameraPlugin;

impl Plugin<ClientPlugin> for CameraPlugin {
    fn create(&self, world: &mut World, _schedule: &mut Schedule) {
        world.insert_resource(MainCamera::default());
    }
}

#[derive(Resource, Default)]
pub struct MainCamera(Camera);

impl std::ops::Deref for MainCamera {
    type Target = Camera;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for MainCamera {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
