use bevy_ecs::prelude::*;
use macroquad::prelude::*;

use starbloom_base::*;
use starbloom_camera::*;
use starbloom_map::*;

mod player;

use crate::player::*;

struct MainPlugin();

impl Plugin for MainPlugin {
    fn create(world: &mut World, _schedule: &mut Schedule) {
        let mut regestry = world
            .get_resource_mut::<TileRegestry>()
            .expect(&"Could not get tile regestry");
        regestry.regester("starbloom:air", Tile::declare(false));

        world.spawn(Player());
    }
}

async fn mainloop() {
    let mut world: World = World::new();
    let mut schedule: Schedule = Schedule::default();

    CameraPlugin::create(&mut world, &mut schedule);
    MapPlugin::create(&mut world, &mut schedule);
    PlayerPlugin::create(&mut world, &mut schedule);
    MainPlugin::create(&mut world, &mut schedule);

    loop {
        clear_background(WHITE);
        schedule.run(&mut world);
        next_frame().await;
    }
}

#[macroquad::main("Starbloom")]
async fn main() {
    set_fullscreen(true);

    #[cfg(feature = "intro")]
    starbloom_intro::main().await;

    mainloop().await;
}
