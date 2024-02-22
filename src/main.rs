#![feature(test)]
#![feature(slice_pattern)]
extern crate core;
extern crate test;

mod compute;
mod graphics;
mod life;
mod perf;

use crate::compute::Compute;
use crate::graphics::scene::Scene;
use crate::graphics::sprite::Sprite;
use crate::graphics::view::Viewable;
use macroquad::prelude::*;
use macroquad::ui;
use std::rc::Rc;
use std::time::Instant;

fn window_conf() -> Conf {
    Conf {
        window_title: "Life Net".to_owned(),
        window_width: 256,
        window_height: 256,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let width = screen_width().floor() as u32;
    let height = screen_height().floor() as u32;

    //convert f32 screen size to something safe for render on image (u16 size)
    //TMP : convert to u16 (until world implement multiquads... TODO)
    let w: u16 = u16::try_from(width).unwrap_or_else(|_v| u16::MAX);
    let h: u16 = u16::try_from(height).unwrap_or_else(|_v| u16::MAX);

    println!("{} {}", w, h);

    //We want a functional architecture
    // => the inner structure of the nested loops' states should probably be reflected here somehow ?

    let life_world = Rc::new(Box::new(life::quad::Quad::new(w, h)));

    let screen = graphics::view::View::new()
        .with_dimensions(width, height)
        .with_framerate(60);

    let scene: Scene<life::quad::Quad> = Scene::new(RED)
        .with_drawable(Box::new(Sprite::from_viewable(Rc::downgrade(&life_world))))
        .with_view(screen); // use World (simulation) as a parameter for Scene(graphics)

    ///Simulation will take ownership of World
    /// TODO : redesign around that... (extract useful parts of the sim runner and use elsewhere...
    // let mut simulation = Simulation::new(life::world::World::new(w, h), 32);
    let mut simulation = compute::discrete::DiscreteTime::new(life_world).with_max_update_rate(5.);

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

    let mut last_update = Instant::now();

    // TODO : generic throttled loop here
    loop {
        let available_sim_duration = screen
            .target_frame_time()
            .saturating_sub(screen.last_frame_time());

        //Note : Discrete simulation can be called multiple time without rendering (speed purposes)
        // However a Continuous simulation (working on floats) leverage the elapsed time to algebraically compute next Update.
        // CAREFUL : Simulation could also be called multiple times, just to finish one full Update...

        // attempt (TODO) multiple total Update on (possibly linear) simulation
        simulation.update(last_update.elapsed(), available_sim_duration);

        // simulation render on screen...
        // simulation.render(screen);
        //TODO : make this implicit somehow...

        last_update = Instant::now();

        //TODO : put this in UI (useful only if different from FPS...)
        let ups = simulation.get_updates_per_second();
        if ups.is_some() {
            ui::root_ui().label(None, &format!("UPS: {}", ups.unwrap()));
        }

        //TODO: which one is best ??
        scene.display().await;
        // screen.update().await;
        // graphics::show(view).await;
    }
}
