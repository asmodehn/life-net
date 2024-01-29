mod engine;
mod life;
mod render;

use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Life Net".to_owned(),
        window_width: 256,
        window_height: 128,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    //convert f32 screen size to something safe for render on image (u16 size)
    let w = screen_width().floor() as usize;
    let h = screen_height().floor() as usize;

    println!("{} {}", w, h);

    //We want a functional architecture
    // => the structure of the nested loops' states should be reflected here

    let mut world = life::World::new(w, h);

    //TMP the render engine is hte holder of world... WIP : change to smthg else...
    let mut re = render::RenderBuffer::new(w as u16, h as u16); // TMP as u16 -> find better ways...

    render::run(&mut re, &mut world).await;
}
