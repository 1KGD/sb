use bevy_ecs::prelude::*;
use macroquad::prelude::*;

use starbloom_base::*;

async fn mainloop() {
    let mut world: World = World::new();
    let mut schedule: Schedule = Schedule::default();

    schedule.add_systems(render_player);

    world.spawn(Player());

    loop { 
        clear_background(WHITE);
        schedule.run(&mut world);
        next_frame().await;
    }
}

#[derive(Component)]
#[require(Position)]
struct Player();

fn render_player(query: Query<&Position, With<Player>>) {
    for position in query {
        draw_rectangle(position.0.x, position.0.y, 20., 20., BLACK);
    }
}

#[macroquad::main("Starbloom")]
async fn main() {
    #[cfg(feature = "intro")]
    starbloom_intro::main().await;

    mainloop().await;
}
