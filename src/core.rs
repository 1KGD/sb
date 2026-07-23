use bevy_ecs::prelude::*;
use egor::{app::*, render::*};

use starbloom_base::*;
use starbloom_camera::*;
use starbloom_map::*;
use starbloom_tiles::*;
use starbloom_worldgen::*;

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

egor::main!(main);
pub fn main() {
    println!("STARBLOOM v{}", VERSION);

    let mut world: World = World::new();
    let mut schedule: Schedule = Schedule::default();

    CameraPlugin::create(&mut world, &mut schedule);
    MapPlugin::create(&mut world, &mut schedule);
    WorldgenPlugin::create(&mut world, &mut schedule);
    PlayerPlugin::create(&mut world, &mut schedule);
    MainPlugin::create(&mut world, &mut schedule);

    App::new().title("STARBLOOM").run(move |ctx: &mut FrameContext<'_>| {
        ctx.gfx.clear(Color::GREEN);
        schedule.run(&mut world);
    });
}
