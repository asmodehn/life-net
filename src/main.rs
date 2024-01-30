mod engine;
mod life;
mod render;
mod simulation;

use crate::simulation::Simulation;
use macroquad::prelude::*;
use std::cell::RefCell;

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

    let mut simulation = Simulation::new(life::world::World::new(w, h), 32);

    //TMP the render engine is hte holder of world... WIP : change to smthg else...
    let re = render::RenderBuffer::new(&simulation.world.image, 60); // TMP as u16 -> find better ways...

    let engine = engine::Engine {
        display: RefCell::new(re),
        simulation: RefCell::new(simulation),
    };

    engine.async_run().await;

    // render::run(&mut re, &mut world).await;

    // API GOAL:
    // display::show(
    //      simulation::run(world, limiter /* some kind of customisable CPU limiter*/),
    //      /*optional gui ? */
    //      /* optional audio ?  ==> HOW ? maybe different API with hecs ?? */
    //      60 /*FPS : the traditional GPU limiter */
    // ).await;
    // OR
    // let engine = Engine{ display: , audio: , simulation: }
    // engine.run().await;
}
