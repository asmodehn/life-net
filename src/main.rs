mod engine;
mod life;
mod render;

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

    //We want a functional architecture
    // => the structure of the nested loops' states should be reflected here

    let mut world = life::World::new(w, h);

    //TMP the render engine is hte holder of world... WIP : change to smthg else...
    let mut re = render::RenderBuffer::new(w, h);

    render::run(&mut re, &mut world).await;
}
