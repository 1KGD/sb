use bevy_ecs::prelude::*;
use macroquad::prelude::*;

use starbloom_base::*;
use starbloom_map::*;

async fn mainloop() {
    let mut world: World = World::new();
    let mut schedule: Schedule = Schedule::default();

    world.insert_resource(TileMap::default());

    schedule.add_systems((update_player, render_player));

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
        draw_rectangle(position.x, position.y, 20., 20., BLACK);
    }
}

fn update_player(query: Query<&mut Position, With<Player>>) {
    for mut position in query {
        if is_key_down(KeyCode::Down) {
            position.y += 10. * get_frame_time();
        }
    }
}

#[macroquad::main("Starbloom")]
async fn main() {
    #[cfg(feature = "intro")]
    starbloom_intro::main().await;

    mainloop().await;
}
