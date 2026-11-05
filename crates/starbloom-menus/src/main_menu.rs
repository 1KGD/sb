use crate::*;

pub(crate) fn render_main_menu(
    renderer: NonSendMut<Renderer>,
    mut state_manager: ResMut<ClientStateManager>,
) {
    egui::Modal::new(egui::Id::new("main_menu")).show(renderer.egui_ctx, |ui: &mut egui::Ui| {
        ui.heading("STARBLOOM");
        ui.label("foo");
        if ui.button("Play").clicked() {
            state_manager.switch(ClientState::Mainloop);
        }
    });
}
