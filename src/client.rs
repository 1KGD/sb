use egor::app::egui::*;
use starbloom_base::prelude::*;
use starbloom_input::prelude::*;

pub fn main(mut world: World, mut schedule: Schedule, modded: bool) {
    world.insert_non_send(Renderer::new());

    let title: String = format!("STARBOOM v{}{}", VERSION, if modded { "*" } else { "" });

    #[cfg(feature = "web")]
    {
        wasm_logger::init(
            wasm_logger::Config::new(if cfg!(feature = "debug_logging") {
                Level::Debug
            } else {
                Level::Info
            })
            .module_prefix("starbloom"),
        );
        console_error_panic_hook::set_once();
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .set_title(&title);
    }

    let mut fonts: FontDefinitions = FontDefinitions::default();
    fonts.font_data.insert(
        "Pixelify Sans".to_owned(),
        FontData::from_static(include_bytes!("../assets/fonts/PS.ttf")).into(),
    );

    let mut newfam: std::collections::BTreeMap<FontFamily, Vec<String>> =
        std::collections::BTreeMap::new();
    newfam.insert(
        FontFamily::Name("Pixelify Sans".into()),
        vec!["Pixelify Sans".to_owned()],
    );
    fonts.families.append(&mut newfam);

    App::new()
        .window_size(256, 256)
        .resizable(false)
        .title(&title)
        .run(move |ctx: &mut FrameContext<'_>| {
            if ctx.timer.frame == 0 {
                ctx.gfx.load_font(include_bytes!("../assets/fonts/PS.ttf"));

                ctx.egui_ctx.set_fonts(fonts.clone());
                ctx.egui_ctx.set_pixels_per_point(0.75);
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
                            FontId::new(16., FontFamily::Name("Pixelify Sans".into())),
                        ),
                        (
                            TextStyle::Body,
                            FontId::new(16., FontFamily::Name("Pixelify Sans".into())),
                        ),
                        (
                            TextStyle::Button,
                            FontId::new(16., FontFamily::Name("Pixelify Sans".into())),
                        ),
                    ]
                    .into();

                    style.visuals.override_text_color = Some(Color32::from_hex("#193d3f").unwrap());
                    style.visuals.window_fill = Color32::from_hex("#ffe762").unwrap();
                    style.visuals.extreme_bg_color = Color32::from_hex("#63c64d").unwrap();
                    style.visuals.striped = true;
                    style.visuals.interact_cursor = Some(CursorIcon::PointingHand);
                    style.interaction.selectable_labels = false;
                    style.visuals.window_highlight_topmost = false;
                    style.visuals.window_stroke.color = Color32::BLACK;
                    style.visuals.window_stroke.width = 0.1;

                    style.spacing.scroll.floating = false;
                });
            }

            world.insert_resource(FrameData::from(&*ctx));
            world.get_non_send_mut::<Renderer<'_>>().unwrap().0 =
                (&raw mut *ctx) as *mut FrameContext<'_>; // I DON'T WANT TO TALK ABOUT IT, OK?
            /*world
                .get_resource_mut::<InputCtx>()
                .unwrap()
                .update(ctx.input);*/

            schedule.run(&mut world);

            #[cfg(feature = "debug_ui")]
            egui::Window::new("Debug").show(ctx.egui_ctx, |ui: &mut Ui| {
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
