use crate::*;

#[derive(Component)]
pub struct MainMenu;

pub(crate) fn render_main_menu(_query: If<Single<&MainMenu>>, renderer: NonSendMut<Renderer>) {
    egui::Window::new("Main Menu").show(&renderer.egui_ctx, |ui: &mut egui::Ui| {
        ui.label("Hello World");
        if ui.button("Don't Click").clicked() {
            panic!("I warned you!");
        }
    });
}
