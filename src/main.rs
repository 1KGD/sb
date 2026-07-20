use bevy_ecs::prelude::*;
use raylib::prelude::*;

use starbloom_base::*;
use starbloom_camera::*;
use starbloom_map::*;
use starbloom_tiles::*;
use starbloom_worldgen::*;

mod player;

use crate::player::*;

struct MainPlugin();

impl Plugin for MainPlugin {
    fn create(world: &mut World, _schedule: &mut Schedule) {
        let mut regestry = world
            .get_resource_mut::<TileRegestry>()
            .expect(&"Could not get tile regestry");
        regestry.regester("starbloom:air", Tile::declare(false));
        regestry.regester("starbloom:debug", Tile::declare(true));

        world.spawn(Player {
            name: "guest".to_owned(),
        });
    }
}

fn main() {
    println!("STARBLOOM v{}", VERSION);

    let mut world: World = World::new();
    let mut schedule: Schedule = Schedule::default();

    CameraPlugin::create(&mut world, &mut schedule);
    MapPlugin::create(&mut world, &mut schedule);
    WorldgenPlugin::create(&mut world, &mut schedule);
    PlayerPlugin::create(&mut world, &mut schedule);
    MainPlugin::create(&mut world, &mut schedule);

    let (mut handle, thread) = raylib::init()
        .fullscreen()
        .title(format!("STARBLOOM v{}", VERSION))
        .build();

    world.insert_non_send(RenderContext::default());

    loop {
        let mut ctx = world.non_send_mut::<RenderContext>();
        ctx.as_mut().drawer = Some(handle.begin_drawing(&thread));
        schedule.run(&mut world);
        ctx.as_mut().drawer = None;
    }
}
