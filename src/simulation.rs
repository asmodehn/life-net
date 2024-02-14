//This is one of a few modules defining the engine:
// - display
// - simulation
// - and more systems later...

use crate::life::quad::Quad;
use crate::perf::DurationAverage;
use crate::render::Renderable;
use macroquad::prelude::Image;
use std::cmp::{max, min};
use std::time::{Duration, Instant};

#[allow(dead_code)]
trait Stepper {
    fn next() -> impl Stepper;
}

#[derive(Debug, PartialEq)]
enum UpdateKind {
    Total,
    Partial,
}

pub(crate) struct Simulation {
    pub(crate) world: Quad, // TODO : that is where we hook hecs or more complex game-world mgmt things...
    max_update_duration: Duration,
    last_second: Instant,
    update_count_since_last_second: u32,
    last_ups: f32,

    average_duration: DurationAverage,
    update: UpdateKind,
}

impl Simulation {
    pub fn new(quad: Quad, max_ups: i32) -> Simulation {
        //TODO: prevent maxups = 0
        let max_update_duration = Duration::from_secs_f32(1. / max_ups as f32);
        println!("MAX_UPDATE_DUR {}", max_update_duration.as_secs_f32());

        Simulation {
            world: quad,
            max_update_duration,
            last_second: Instant::now(),
            update_count_since_last_second: 0,
            last_ups: 0.,
            average_duration: DurationAverage::default(),
            update: UpdateKind::Total,
        }
    }

    fn ups_count(&mut self) {
        if self.last_second.elapsed() > Duration::new(1, 0) {
            self.last_ups = self.update_count_since_last_second as f32
                / self.last_second.elapsed().as_secs_f32();
            self.last_second = Instant::now();
            self.update_count_since_last_second = 0;
        }
        self.update_count_since_last_second += 1;
    }

    pub(crate) fn update(&mut self, elapsed: Duration, available: Duration) {
        // measurement of ups is here since this method is called repeatedly by the engine.
        self.ups_count();
        // if total is impossible, Simulation will do partial if implemented/possible...
        // => fractional update count possible

        // self.world.update(elapsed, available);

        self.average_duration.timed_start();

        if self.average_duration.avg().unwrap_or_default() < available {
            if self.update == UpdateKind::Partial {
                // println!("{:?} => ATTEMPT COMPLETE UPDATE IN ONE CALL", available);
                self.update = UpdateKind::Total;
            }

            self.world
                .update(elapsed, available, |quad: &Quad| quad.completed());

            // self.next
            //     .compute(&mut self.image, |partial_quad: &crate::life::quad::partial::PartialQuad| {
            //         partial_quad.is_ready()
            //     });
        } else {
            if self.update == UpdateKind::Total {
                // println!("{:?} => NOT ENOUGH TIME FOR FULL UPDATE", available);
                self.update = UpdateKind::Partial;
            }
            self.world.update(elapsed, available, |_quad: &Quad| {
                self.average_duration.timed_elapsed() >= available
            });

            // self.next
            //     .compute(&mut self.image, |_partial_quad: &crate::life::quad::partial::PartialQuad| {
            //         self.average_duration.timed_elapsed() > available
            //     });
        }

        self.average_duration.timed_stop();
    }

    pub fn get_ups(&self) -> f32 {
        self.last_ups
    }
}

impl Renderable for Simulation {
    fn render(&mut self) -> &Image {
        // self.world.render()
        // no need to render with a quad
        &self.world.image // TODO : call swapbuf here, instead of every update...
    }
}

#[cfg(test)]
mod tests {
    use crate::life::cell;
    use crate::life::quad::Quad;
    use std::time::Duration;

    use crate::simulation::Simulation;
    use test::Bencher;

    #[test]
    fn lonely_dying_quad() {
        let mut q = Quad::new(1, 1);
        q.image.update(&[cell::ALIVE]);

        let mut s = Simulation::new(q, 1);

        //one update
        s.update(Duration::new(0, 0), Duration::MAX);

        assert_eq!(s.world.image.get_pixel(0, 0), cell::DEAD)
    }

    #[test]
    fn check_stationary_one() {
        let mut q = Quad::new(2, 2);
        //permanent square in quad
        q.image.update(&[cell::ALIVE; 4]);

        let mut s = Simulation::new(q, 1);

        //one update
        s.update(Duration::new(0, 0), Duration::MAX);

        assert_eq!(s.world.image.get_pixel(0, 0), cell::ALIVE);
        assert_eq!(s.world.image.get_pixel(0, 1), cell::ALIVE);
        assert_eq!(s.world.image.get_pixel(1, 0), cell::ALIVE);
        assert_eq!(s.world.image.get_pixel(1, 1), cell::ALIVE);
    }
}
