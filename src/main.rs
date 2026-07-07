use macroquad::prelude::*;

async fn mainloop() {
    loop {
        clear_background(RED);
        next_frame().await;
    }
}

#[macroquad::main("Starbloom")]
async fn main() {
    starbloom_intro::main().await;
    mainloop().await;
}
