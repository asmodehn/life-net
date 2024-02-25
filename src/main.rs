#![feature(test)]
#![feature(slice_pattern)]
extern crate core;
extern crate test;

mod compute;
mod graphics;
mod life;

use crate::compute::discrete::DiscreteTime;
use crate::compute::rate_limiter::RateLimiter;
use crate::compute::Compute;
use crate::compute::{Computable, PartialComputable};
use crate::graphics::sprite::Sprite;
use crate::graphics::view::Viewable;
use macroquad::prelude::*;
use macroquad::ui;
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

    // let mut simulation = Simulation::new(life::world::World::new(w, h), 32);
    // let mut simulation = compute::discrete::DiscreteTime::new(life::quad::Quad::new(w, h))
    //     .with_limiter(RateLimiter::default().with_maximum_rate(5.));

    // let sprite = graphics::sprite::Sprite::from_image(simulation.render());

    //NOT USe anymore
    // let mut screen = graphics::view::View::new(&simulation.world.image, 60);

    // TODO : View ==> Scene

    // TODO : scene, for all relative positioning...

    let mut lifequad = life::quad::Quad::new(w, h);
    let mut sprite = Sprite::from_image(lifequad.render());

    // let mut quadactor: Actor<life::quad::Quad, Sprite> = Actor::new(lifequad, sprite );

    let mut compute_context = compute::ComputeCtx::default();
    //TODO : add limiter to compute context here...

    // TODO : generic throttled loop here
    loop {
        let available_sim_duration =
            graphics::target_frame_time(60.0).saturating_sub(graphics::last_frame_time());

        //Note : Discrete simulation can be called multiple time without rendering (speed purposes)
        // However a Continuous simulation (working on floats) leverage the elapsed time to algebraically compute next Update.
        // CAREFUL : Simulation could also be called multiple times, just to finish one full Update...

        // attempt (TODO) multiple total Update on (possibly linear) simulation
        // lifeworld.compute_partial(last_update.elapsed(), Some(available_sim_duration));

        compute_context.set_constraint(available_sim_duration);

        compute_context = compute::compute_partial(&mut lifequad, compute_context);

        //TODO : put this in UI (useful only if different from FPS...)
        // let ups = simulation.get_updates_per_second();
        // if ups.is_some() {
        //     ui::root_ui().label(None, &format!("UPS: {}", ups.unwrap()));
        // }

        // screen.update(&mut simulation).await;

        graphics::update(&mut sprite, &lifequad);

        graphics::render(&sprite, IVec2::new(0, 0)).await;
    }
}
