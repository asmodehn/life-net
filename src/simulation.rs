//This is one of a few modules defining the engine:
// - display
// - simulation
// - and more systems later...

use crate::life::world::World;
use crate::render::{RenderBuffer, Renderable};
use macroquad::prelude::Image;
use std::cmp::{max, min};
use std::time::{Duration, Instant};

pub(crate) struct Simulation {
    last_now: Instant,
    pub(crate) world: World, // TODO : that is where we hook hecs or more complex game-world mgmt things...
    max_update_duration: Duration,
    last_second: Instant,
    update_count_since_last_second: u32,
}

impl Simulation {
    pub fn new(world: World, max_ups: i32) -> Simulation {
        //TODO: prevent maxups = 0
        let max_update_duration = Duration::from_secs_f32(1. / max_ups as f32);
        println!("MAX_UPDATE_DUR {}", max_update_duration.as_secs_f32());

        Simulation {
            last_now: Instant::now(),
            world,
            max_update_duration,
            last_second: Instant::now(),
            update_count_since_last_second: 0,
        }
    }
    pub(crate) fn update(&mut self) {
        let updated = self.world.update(self.last_now.elapsed());

        self.last_now = Instant::now();

        updated
    }
    /// runs update() the simulation for a certain duration
    /// minimum is one update() call.
    pub fn run(&mut self, available_duration: Duration) {
        // measurement of ups is here since this method is called repeatedly by the engine.
        if self.last_second.elapsed() > Duration::new(1, 0) {
            self.last_second = Instant::now();
            self.update_count_since_last_second = 0;
        }

        let start = Instant::now();

        let max_duration = min(available_duration, self.max_update_duration);
        // println!("MAX_DUR {}", max_duration.as_secs_f32());
        let mut calls = 0;

        while calls == 0 || start.elapsed() <= max_duration {
            calls += 1;
            self.update();
        }

        self.update_count_since_last_second += calls;

        // println!("CALLS: {}", calls);
    }

    pub fn get_ups(&self) -> i32 {
        (self.update_count_since_last_second as f32 / self.last_second.elapsed().as_secs_f32())
            as i32
    }
}

impl Renderable for Simulation {
    fn render(&mut self) -> &Image {
        self.world.render()
    }
}
