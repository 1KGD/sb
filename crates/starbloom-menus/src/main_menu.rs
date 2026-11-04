use crate::*;

#[derive(Component)]
pub struct MainMenu;

pub(crate) fn render_main_menu(query: If<Single<&MainMenu>>, renderer: NonSendMut<Renderer>) {
    let menu: &MainMenu = &query;

    egui::Window::new("Main Menu").show(&renderer.egui_ctx, |ui: &mut egui::Ui| {
        ui.label("Hello World");
    });
}
