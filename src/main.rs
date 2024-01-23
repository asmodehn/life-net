mod life;

use std::thread;
use std::time::Duration;
use macroquad::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum CellState {
    Alive,
    Dead,
}


fn window_conf() -> Conf {
    Conf {
        window_title: "Life Net".to_owned(),
        window_width: 128,
         window_height: 128,
         fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let w = screen_width() as usize;
    let h = screen_height() as usize;

    let mut cells = vec![CellState::Dead; w * h];
    let mut buffer = vec![CellState::Dead; w * h];

    let mut image = Image::gen_image_color(w as u16, h as u16, WHITE);

    for cell in cells.iter_mut() {
        if rand::gen_range(0, 5) == 0 {
            *cell = CellState::Alive;
        }
    }
    let texture = Texture2D::from_image(&image);

    let mut last_game_tick = get_time();

    let target_fps = 60. as f64;
    let target_frame_time = (1. / target_fps) as f64;

    loop {
        // WIP : manage cpu usage via timer ie. with fps limiter
        let game_tick = get_time();

        let tts = target_frame_time - game_tick + last_game_tick;

        // if new frame happens too early, we skip it

        if tts > 0. {
            // sleep on app if needed...
            println!("sleep {:?}", tts);
            thread::sleep(Duration::from_secs_f64(tts));
        }

        // OR wait for next frame
            // works to skip update in wasm but breaks render on native app
            // next_frame().await
            //See https://github.com/not-fl3/macroquad/issues/170 and https://github.com/not-fl3/macroquad/issues/380
        // } else {

            last_game_tick = game_tick;

            clear_background(WHITE);

            let w = image.width();
            let h = image.height();

            for y in 0..h as i32 {
                for x in 0..w as i32 {
                    buffer[y as usize * w + x as usize] = life::cell_update(&cells, x, y, w, h);
                }
            }

            for i in 0..buffer.len() {
                cells[i] = buffer[i];

                image.set_pixel(
                    (i % w) as u32,
                    (i / w) as u32,
                    match buffer[i as usize] {
                        CellState::Alive => BLACK,
                        CellState::Dead => WHITE,
                    },
                );
            }

            texture.update(&image);

            draw_texture(&texture, 0., 0., WHITE);

            next_frame().await
        // }
    }
}