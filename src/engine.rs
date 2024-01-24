use macroquad::color::WHITE;
use macroquad::prelude::{clear_background, draw_texture, get_time, next_frame, Image, Texture2D};
use std::thread;
use std::time::Duration;

pub struct RenderBuffer {
    pub image: Image,
    texture: Texture2D,
}
//TODO : maybe get rid of this ??

pub trait Renderable {
    fn render(&self, buffer: &mut Image);
}

pub trait Updateable {
    fn update(&mut self /* delta time */);
}

pub fn init(screen_width: usize, screen_height: usize) -> RenderBuffer {
    let img = Image::gen_image_color(screen_width as u16, screen_height as u16, WHITE);
    let txtr = Texture2D::from_image(&img);
    RenderBuffer {
        image: img,
        texture: txtr,
    }
}

pub async fn throttled_loop<T: Updateable + Renderable>(
    init_render: &mut RenderBuffer,
    init_gamestate: &mut T,
) {
    let mut last_game_tick = get_time();

    let target_fps = 60. as f64;
    let target_frame_time = (1. / target_fps) as f64;

    let world = init_gamestate;
    let buffer = init_render;

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

        // GAME UPDATE START
        world.update();

        //GAME UPDATE END

        world.render(&mut buffer.image);

        buffer.texture.update(&buffer.image);

        draw_texture(&buffer.texture, 0., 0., WHITE);

        next_frame().await;
        // }
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::{engine, CellState};
//
//     #[test]
//     fn check_fps_throttle() {
//
//         // TODO : some global time...
//         fn timecheck(){
//
//         }
//
//
//         engine::throttled_loop(timecheck);
//
//
//     }
// }
