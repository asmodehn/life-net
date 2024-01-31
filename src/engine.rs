use crate::render::{RenderBuffer, Renderable};
use crate::simulation::Simulation;
use macroquad::prelude::{get_frame_time, next_frame};
use std::cell::RefCell;
use std::time::Duration;

/// This is a struct containing all the side-effects possible in the engine
pub(crate) struct Engine {
    pub display: RefCell<RenderBuffer>,
    pub simulation: RefCell<Simulation>, //TODO : rate... time limit, etc.
}

impl Engine {
    pub async fn async_run(&self) {
        //internal mutability for engine parts
        let mut screen = self.display.borrow_mut();
        let mut simulation = self.simulation.borrow_mut();

        // calling old render runner
        // render::run(
        //     &mut screen,
        //     &mut world
        // ).await
        // TODO : generic throttled loop here
        loop {
            let available_sim_duration = screen
                .target_frame_time()
                .saturating_sub(screen.last_frame_time());

            //TMP render update driving simulation update
            // simulation.update();

            //TODO : call run instead...
            simulation.run(available_sim_duration);

            //TMP : render here instead of update ? because we want to avoid hold world ref in renderbuffer ?
            screen.update(simulation.render()).await;

            // println!("FPS: {}", screen.current_fps());

            // println!("UPS: {}", simulation.get_ups());
        }
    }
}

//
//
// //Todo : nestable engines...
//
// // pub async fn throttled_loop<T: Updatable + Renderable>(
// //     init_render: &mut RenderEngine,
// //     init_gamestate: &mut T,
// // ) {
// //     let mut last_game_tick = get_time();
// //
// //     let target_fps = 60. as f64;
// //     let target_frame_time = (1. / target_fps) as f64;
// //
// //     let world = init_gamestate;
// //     let buffer = init_render;
// //
// //     loop {
// //         // WIP : manage cpu usage via timer ie. with fps limiter
// //         let game_tick = get_time();
// //
// //         let tts = target_frame_time - game_tick + last_game_tick;
// //
// //         // if new frame happens too early, we skip it
// //
// //         if tts > 0. {
// //             // sleep on app if needed...
// //             println!("sleep {:?}", tts);
// //             thread::sleep(Duration::from_secs_f64(tts));
// //         }
// //
// //         // OR wait for next frame
// //         // works to skip update in wasm but breaks render on native app
// //         // next_frame().await
// //         //See https://github.com/not-fl3/macroquad/issues/170 and https://github.com/not-fl3/macroquad/issues/380
// //         // } else {
// //
// //         last_game_tick = game_tick;
// //
// //         clear_background(WHITE);
// //
// //         //TODO : separate update refresh rate and render rate...
// //         world.update(Duration::from_secs_f32(get_frame_time()));
// //
// //         buffer.render(world);
// //
// //         next_frame().await;
// //         // }
// //     }
// // }
//
// // #[cfg(test)]
// // mod tests {
// //
// //     use std::cell::Cell;
// //     use std::time::{Duration, Instant};
// //
// //     // use test::Bencher;
// //     use crate::engine::{Renderable, Updatable};
// //     use macroquad::prelude::Image;
// //
// //     //Extensive timer test, to be able to verify various rendering strategies
// //     //and various frame limiters without changing much in test code
// //
// //     struct RenderableState {
// //         pub since_last_render: Cell<Instant>,
// //         pub elapsed: Duration,
// //         pub engine_elapsed: Duration,
// //     }
// //
// //     impl RenderableState {
// //         fn new() -> RenderableState {
// //             RenderableState {
// //                 since_last_render: Cell::new(Instant::now()),
// //                 elapsed: Duration::new(0, 0),
// //                 engine_elapsed: Duration::new(0, 0),
// //             }
// //         }
// //     }
// //     impl Renderable for RenderableState {
// //         fn render<'s>(&'s self, buffer: &'s mut Image) -> &'s Image{
// //             // Finally print last computed elapsed duration
// //             println!("Elapsed: {:.2?}", self.elapsed);
// //
// //             // Restart counter to calculate next elapsed duration
// //             self.since_last_render.set(Instant::now());
// //
// //             //TODO : some simple render for bench...
// //             buffer
// //         }
// //     }
// //
// //     impl Updatable for RenderableState {
// //         fn update(&mut self, elapsed: Duration) {
// //             self.elapsed = self.since_last_render.get().elapsed();
// //             self.engine_elapsed = elapsed;
// //         }
// //     }
// //     //
// //     // #[test]
// //     // fn test_throttled_loop() {
// //     //     let mut rb = engine::RenderBuffer::new(1,1);
// //     //     let mut rstate = RenderableState::new();
// //     //     engine::throttled_loop(&mut rb, &mut rstate).await;
// //     //
// //     //     //TODO : verify framerate remains "acceptable", over a few frames...
// //     //
// //     // }
// //     //
// //     // #[bench]
// //     // async fn bench_render(b: &mut Bencher) {
// //     //     let mut rb = engine::RenderBuffer::new(1,1);
// //     //     let mut rstate = RenderableState::new();
// //     //
// //     //     //TODO : bench this multiple time, with multiple buffer sizes...
// //     //     rb.render(&rstate);
// //     //
// //     // }
// // }
