mod engine;
mod life;

use macroquad::prelude::*;

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

    let mut rb = engine::RenderBuffer::new(w, h);

    let mut world = life::World::new(w, h);

    engine::throttled_loop(&mut rb, &mut world).await;
}
