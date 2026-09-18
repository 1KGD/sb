use egor::app::egui::*;
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

    BootstrapPlugin::create(&mut world, &mut schedule);
    InputPlugin::create(&mut world, &mut schedule);
    CameraPlugin::create(&mut world, &mut schedule);
    MapPlugin::create(&mut world, &mut schedule);
    DefaultTilePlugin::create(&mut world, &mut schedule);
    WorldgenPlugin::create(&mut world, &mut schedule);
    PlayerPlugin::create(&mut world, &mut schedule);
    MainPlugin::create(&mut world, &mut schedule);

    world.insert_non_send(Renderer::new());

    let mut fonts: FontDefinitions = FontDefinitions::default();
    fonts.font_data.insert(
        "PS".to_owned(),
        FontData::from_static(include_bytes!("../assets/fonts/PS.ttf")).into(),
    );

    let mut newfam: std::collections::BTreeMap<FontFamily, Vec<String>> =
        std::collections::BTreeMap::new();
    newfam.insert(FontFamily::Name("PS".into()), vec!["PS".to_owned()]);
    fonts.families.append(&mut newfam);

    App::new()
        .window_size(256, 256)
        .fullscreen(true)
        .title(&format!("STARBOOM v{}", VERSION))
        .run(move |mut ctx: &mut FrameContext<'_>| {
            if ctx.timer.frame == 0 {
                ctx.egui_ctx.set_fonts(fonts.clone());
                ctx.egui_ctx.set_pixels_per_point(0.5);
            } else if ctx.timer.frame == 1 {
                ctx.egui_ctx.tessellation_options_mut(
                    |options: &mut epaint::TessellationOptions| {
                        options.feathering = false;
                        options.round_line_segments_to_pixels = true;
                        options.round_rects_to_pixels = true;
                        options.round_text_to_pixels = true;
                    },
                );
                ctx.egui_ctx.style_mut(|style: &mut egui::Style| {
                    style.text_styles = [
                        (
                            TextStyle::Heading,
                            FontId::new(24., FontFamily::Name("PS".into())),
                        ),
                        (
                            TextStyle::Body,
                            FontId::new(20., FontFamily::Name("PS".into())),
                        ),
                    ]
                    .into();
                });
            }

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
                    ctx.timer.fps,
                    (ctx.timer.delta * 1000.).round() / 1000.,
                    ctx.timer.frame
                ));
            });

            world.get_non_send_mut::<Renderer<'_>>().unwrap().0 = std::ptr::null_mut(); // Look, I'm being safe, OK?
        });
}
