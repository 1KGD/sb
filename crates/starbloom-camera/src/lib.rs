use bevy_ecs::prelude::*;
use macroquad::prelude::*;

use starbloom_base::*;

pub struct CameraPlugin();

impl Plugin for CameraPlugin {
    fn create(world: &mut World, schedule: &mut Schedule) {
        world.insert_resource(MainCamera::default());
        schedule.add_systems(prepare_camera);
    }
}

#[derive(Resource, Default)]
pub struct MainCamera {
    pub position: Vec2,
    pub rotation: f32,
}

pub fn prepare_camera(camera: Res<MainCamera>) {
    let mut cam = Camera2D::from_display_rect(Rect::new(
        0.,
        screen_height(),
        screen_width(),
        -screen_height(),
    ));

    cam.target = camera.position;
    cam.rotation = camera.rotation;

    set_camera(&cam);
}
