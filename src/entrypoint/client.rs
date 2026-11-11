use starbloom_core_client::prelude::*;
use starbloom_core_shared::prelude::*;

fn compute_offscreen_texture_scale(screen_size: Vec2) -> (Vec2, Vec2) {
    let dim: Vec2 =
        if screen_size.x * MIN_DIMENSION.y as f32 <= screen_size.y * MIN_DIMENSION.x as f32 {
            vec2(
                screen_size.x,
                MIN_DIMENSION.y as f32 * screen_size.x / MIN_DIMENSION.x as f32,
            )
        } else {
            vec2(
                MIN_DIMENSION.x as f32 * screen_size.y / MIN_DIMENSION.y as f32,
                screen_size.y,
            )
        };
    return (dim, (screen_size - dim) / 2.);
}

pub fn main(mut world: World, mut schedule: Schedule, modded: bool) {
    let title: String = format!("STARBOOM v{}{}", VERSION, if modded { "*" } else { "" });

    #[cfg(feature = "web")]
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .set_title(&title);

    debug!("IS_MOBILE = {}", *IS_MOBILE);
    world.insert_non_send(Egui::new());
    world.insert_non_send(Renderer::new());

    let mut fonts: egui::FontDefinitions = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "Pixelify Sans".to_owned(),
        egui::FontData::from_static(include_bytes!("../../assets/fonts/PS.ttf")).into(),
    );

    let mut newfam: std::collections::BTreeMap<egui::FontFamily, Vec<String>> =
        std::collections::BTreeMap::new();
    newfam.insert(
        egui::FontFamily::Name("Pixelify Sans".into()),
        vec!["Pixelify Sans".to_owned()],
    );
    fonts.families.append(&mut newfam);

    let mut offscreen: Option<OffscreenTarget> = None;

    App::new()
        .title(&title)
        .run(move |ctx: &mut FrameContext<'_>| {
            if ctx.timer.frame == 0 {
                offscreen = Some(
                    ctx.gfx
                        .create_offscreen(MIN_DIMENSION.x as u32, MIN_DIMENSION.y as u32),
                );

                ctx.gfx.load_font(include_bytes!("../../assets/fonts/PS.ttf"));

                ctx.egui_ctx.set_fonts(fonts.clone());
            } else if ctx.timer.frame == 1 {
                ctx.egui_ctx.tessellation_options_mut(
                    |options: &mut egui::epaint::TessellationOptions| {
                        options.round_line_segments_to_pixels = true;
                        options.round_rects_to_pixels = true;
                        options.round_text_to_pixels = true;
                    },
                );
                ctx.egui_ctx.style_mut(|style: &mut egui::Style| {
                    style.text_styles = [
                        (
                            egui::TextStyle::Heading,
                            egui::FontId::new(32., egui::FontFamily::Name("Pixelify Sans".into())),
                        ),
                        (
                            egui::TextStyle::Body,
                            egui::FontId::new(24., egui::FontFamily::Name("Pixelify Sans".into())),
                        ),
                        (
                            egui::TextStyle::Button,
                            egui::FontId::new(24., egui::FontFamily::Name("Pixelify Sans".into())),
                        ),
                    ]
                    .into();

                    style.visuals.override_text_color =
                        Some(egui::Color32::from_hex("#193d3f").unwrap());
                    style.visuals.window_fill = egui::Color32::from_hex("#ffe762").unwrap();
                    style.visuals.extreme_bg_color = egui::Color32::from_hex("#63c64d").unwrap();
                    style.visuals.striped = true;
                    style.visuals.interact_cursor = Some(egui::CursorIcon::PointingHand);
                    style.interaction.selectable_labels = false;
                    style.visuals.window_highlight_topmost = false;
                    style.visuals.window_stroke.color = egui::Color32::BLACK;
                    style.visuals.window_stroke.width = 1.;

                    style.spacing.scroll.floating = false;
                });
            }

            world.insert_resource(FrameData::from(&*ctx));

            world.get_non_send_mut::<Egui>().unwrap().0 =
                (&raw const *ctx.egui_ctx) as *mut egui::Context;

            ctx.gfx
                .render_offscreen(offscreen.as_mut().unwrap(), |gfx: &mut Graphics<'_>| {
                    world.get_non_send_mut::<Renderer<'_>>().unwrap().0 =
                        (&raw mut *gfx) as *mut Graphics<'_>; // I DON'T WANT TO TALK ABOUT IT, OK?

                    world
                        .get_resource_mut::<InputCtx>()
                        .unwrap()
                        .update(ctx.input);

                    schedule.run(&mut world);

                    world.get_non_send_mut::<Renderer<'_>>().unwrap().0 = std::ptr::null_mut(); // Look, I'm being safe, OK?
                });
            world.get_non_send_mut::<Egui>().unwrap().0 = std::ptr::null_mut();

            let offscreen_texture: usize =
                ctx.gfx.offscreen_as_texture(offscreen.as_mut().unwrap());

            #[cfg(feature = "debug_ui")]
            egui::Window::new("Debug").show(ctx.egui_ctx, |ui: &mut egui::Ui| {
                ui.label(format!(
                    "FPS: {}\nDELTA: {}\nFRAME: {}",
                    ctx.timer.fps,
                    (ctx.timer.delta * 1000.).round() / 1000.,
                    ctx.timer.frame
                ));
            });
            ctx.gfx.clear(Color::BLACK);
            let (dim, pos) = compute_offscreen_texture_scale(ctx.gfx.screen_size());
            ctx.gfx.rect().texture(offscreen_texture).size(dim).at(pos);
        });
}
