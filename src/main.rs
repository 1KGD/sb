use bevy_ecs::prelude::*;
use macroquad::prelude::*;

use starbloom_base::*;
use starbloom_map::*;

mod player;

use crate::player::*;

async fn mainloop() {
    let mut world: World = World::new();
    let mut schedule: Schedule = Schedule::default();

    MapPlugin::create(&mut world, &mut schedule);
    PlayerPlugin::create(&mut world, &mut schedule);

    let mut checked: bool = false;
    loop {
        clear_background(WHITE);
        schedule.run(&mut world);
        if !checked {
            if macroquad::ui::root_ui().button(None, "Click Me") {
                checked = true;
            }
        }
        next_frame().await;
    }
}

#[macroquad::main("Starbloom")]
async fn main() {
    set_fullscreen(true);

    #[cfg(feature = "intro")]
    starbloom_intro::main().await;

    mainloop().await;
}
