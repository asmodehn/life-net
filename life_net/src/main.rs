#![feature(test)]
#![feature(slice_pattern)]
extern crate core;
extern crate test;

use figment::compute;
use figment::graphics;
use figment::graphics::Viewable; // needed for render method...
use quadlife::cell;
use macroquad::prelude::*;
use std::ops::Deref;
use std::time::Duration;

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

const PARTIAL_UPDATE: bool = true;

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

    // TODO : View ==> Scene

    // TODO : scene, for all relative positioning...

    let mut lifequad = quadlife::quad::Quad::gen(cell::State::Dead, w, h).with_random_cells();
    let mut sprite = graphics::sprite::Sprite::from_image(lifequad.render().borrow().deref());

    let mut compute_context =
        compute::ComputeCtx::default().with_constraint(Duration::from_secs_f32(1. / 60.));

    let mut quad_upd_opt = if PARTIAL_UPDATE {
        Some(compute::compute_reset(&lifequad))
    } else {
        None
    };

    loop {
        let available_sim_duration =
            graphics::target_frame_time(60.0).saturating_sub(graphics::last_frame_time());

        //Note : Discrete simulation can be called multiple time without rendering (speed purposes)
        // However a Continuous simulation (working on floats) leverage the elapsed time to algebraically compute next Update.
        // CAREFUL : Simulation could also be called multiple times, just to finish one full Update...

        // attempt (TODO) multiple total Update on (possibly linear) simulation
        // lifeworld.compute_partial(last_update.elapsed(), Some(available_sim_duration));

        if PARTIAL_UPDATE {
            //PARTIAL UPDATE
            compute_context.set_constraint(available_sim_duration);

            compute::compute_partial(&mut lifequad, &mut compute_context, &mut quad_upd_opt);
        } else {
            //FULL UPDATE
            compute::compute(&mut lifequad);
        }

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
