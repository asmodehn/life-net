use crate::perf::DurationAverage;
use crate::render::{RenderBuffer, Renderable};
use crate::simulation::Simulation;
use std::cell::RefCell;
use std::time::{Duration, Instant};

// use crate::perf::PerfCounter;

/// This is a struct containing all the side-effects possible in the engine
pub(crate) struct Engine {
    pub display: RefCell<RenderBuffer>,
    pub simulation: RefCell<Simulation>,
    //TODO : audio, physics, etc.

    //TODO : rate... time limit, etc.
    sim_rate: DurationAverage,
    render_rate: DurationAverage,
}

impl Engine {
    pub fn new(display: RenderBuffer, simulation: Simulation) -> Engine {
        Engine {
            display: RefCell::new(display),
            simulation: RefCell::new(simulation),
            sim_rate: DurationAverage::default(),
            render_rate: DurationAverage::default(),
        }
    }

    pub async fn async_run(&mut self) {
        //internal mutability for engine parts
        let mut screen = self.display.borrow_mut();
        let mut simulation = self.simulation.borrow_mut();

        let mut last_update = Instant::now();

        let mut last_ups_update = Instant::now();

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

            // if last_ups_update.elapsed() >

            last_update = Instant::now();

            if last_ups_update.elapsed() > Duration::new(1, 0) {
                println!("UPS: {}", simulation.get_ups());
                last_ups_update = Instant::now();
            }

            screen.update(simulation.render()).await;
        }
    }
}

//TODO : test to verify avialable time for update...
