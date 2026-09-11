use starbloom_base::prelude::*;
use starbloom_camera::*;
use starbloom_input::prelude::*;
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

        world.spawn(RemotePlayer {
            name: "Guest".to_owned(),
        });
    }
}

pub fn main() {
    #[cfg(target_arch = "wasm32")]
    {
        wasm_logger::init(wasm_logger::Config::default().module_prefix("starbloom"));
        console_error_panic_hook::set_once();
    }
    #[cfg(not(target_arch = "wasm32"))]
    env_logger::builder()
        .format_timestamp(None)
        .filter(Some("starbloom"), LevelFilter::Info)
        .init();

    info!("STARBLOOM v{}", VERSION);
    debug!("IS_MOBILE = {}", *IS_MOBILE);

    let mut world: World = World::new();
    let mut schedule: Schedule = Schedule::default();

    world.insert_resource(AssetServer::default());

    InputPlugin::create(&mut world, &mut schedule);
    CameraPlugin::create(&mut world, &mut schedule);
    MapPlugin::create(&mut world, &mut schedule);
    WorldgenPlugin::create(&mut world, &mut schedule);
    PlayerPlugin::create(&mut world, &mut schedule);
    MainPlugin::create(&mut world, &mut schedule);

    world.insert_non_send(Renderer::new());

    App::new().title(&format!("STARBOOM v{}", VERSION)).run(
        move |mut ctx: &mut FrameContext<'_>| {
            world.get_non_send_mut::<Renderer<'_>>().unwrap().0 =
                [&mut ctx].as_mut_ptr() as *mut &mut &mut FrameContext; // I DON'T WANT TO TALK ABOUT IT, OK?
            world
                .get_resource_mut::<InputCtx>()
                .unwrap()
                .update(ctx.input);

            world
                .get_non_send_mut::<Renderer<'_>>()
                .unwrap()
                .ctx()
                .unwrap()
                .gfx
                .clear(Color::BLUE);

            schedule.run(&mut world);

            #[cfg(feature = "debug_ui")]
            egui::Window::new("Debug").show(ctx.egui_ctx, |ui| {
                ui.label(format!(
                    "FPS: {}\nDELTA: {}\nFRAME: {}",
                    ctx.timer.fps, ctx.timer.delta, ctx.timer.frame
                ));
            });
        },
    );
}
