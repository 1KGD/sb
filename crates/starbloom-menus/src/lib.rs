use starbloom_core_client::prelude::*;
use starbloom_core_shared::prelude::*;

mod main_menu;

pub use main_menu::*;

pub struct MenusPlugin;

impl Plugin<ClientPlugin> for MenusPlugin {
    fn create(&self, world: &mut World, schedule: &mut Schedule) {
        schedule.add_systems(render_main_menu.in_set(ClientStep::RenderUI));
        world.spawn(MainMenu);
    }
}
