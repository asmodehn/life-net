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
    ups: i32,
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
            ups: 0,
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
        let start = Instant::now();

        let max_duration = min(available_duration, self.max_update_duration);
        println!("MAX_DUR {}", max_duration.as_secs_f32());
        let mut calls = 0;

        while calls == 0 || start.elapsed() <= max_duration {
            calls += 1;
            self.update();
        }

        println!("CALLS: {}", calls);
        //TODO : fix calculation : doesnt take in account other stuff (rendering, etc...)
        // => average over an entire second !

        {
            self.ups = (calls as f32 / start.elapsed().as_secs_f32()) as i32;
        }
    }

    pub fn get_ups(&self) -> i32 {
        self.ups
    }
}

impl Renderable for Simulation {
    fn render(&mut self) -> &Image {
        self.world.render()
    }
}
