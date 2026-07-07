use macroquad::prelude::*;

async fn screen(duration: f32, render_func: impl AsyncFn(f32)) {
    let mut progress: f32 = 0.;

    loop {
        progress += get_frame_time();
        if progress > duration {
            break;
        }
        render_func(progress / duration).await;
        next_frame().await;
    }
}

const PRESENTS_TEXT: &str = "1KGD\nPresents:";
const TITLE_TEXT: &str = "STARBLOOM";

pub async fn main() {
    screen(5., async |progress: f32| {
        clear_background(BLACK);
        let dim: TextDimensions = measure_multiline_text(PRESENTS_TEXT, None, 40, 1., None);
        draw_multiline_text(
            PRESENTS_TEXT,
            screen_width() / 2. - dim.width / 2.,
            screen_height() / 2. - dim.height / 2.,
            40.,
            None,
            WHITE.with_alpha(1.5 * f32::sin(progress * core::f32::consts::PI * 1.1)),
        );
    })
    .await;

    screen(5., async |progress: f32| {
        clear_background(BLACK);
        let dim: TextDimensions = measure_multiline_text(TITLE_TEXT, None, 100, 1., None);
        draw_multiline_text(
            TITLE_TEXT,
            screen_width() / 2. - dim.width / 2.,
            screen_height() / 2. - dim.height / 2.,
            100.,
            None,
            ORANGE.with_alpha(1.5 * f32::sin(progress * core::f32::consts::PI * 1.1)),
        );
    })
    .await;
}
