use crate::compute::rate_limiter::RateLimiter;
use crate::compute::timer::Timer;
use crate::compute::Compute;
use crate::graphics::view::Viewable;
use crate::life::quad::Quad;
use macroquad::texture::Image;
use std::cmp::min;
use std::time::{Duration, Instant};

//TODO : make it just a struct somehow ??
pub(crate) struct DiscreteTime {
    pub world: Quad,
    limiter: RateLimiter,
    full_update_timer: Timer,
}

impl DiscreteTime {
    pub fn new(quad: Quad) -> DiscreteTime {
        DiscreteTime {
            world: quad,
            limiter: RateLimiter::default(),
            full_update_timer: Timer::default(),
        }
    }
    pub(crate) fn with_limiter(self: Self, limiter: RateLimiter) -> Self {
        Self { limiter, ..self }
    }
}

impl Viewable for DiscreteTime {
    fn render(&self) -> &Image {
        // self.world.render()
        // no need to render with a quad
        &self.world.image
    }
}

//TODO : move this somewher else  !!!
// impl Compute for DiscreteTime {
//     fn update_timer_tick(&mut self) {
//         self.limiter
//             .average_duration
//             .record(self.full_update_timer.elapsed_and_reset())
//     }
//
//     fn get_updates_per_second(&self) -> Option<f32> {
//         self.limiter
//             .average_duration
//             .average()
//             .and_then(|d: Duration| Some(1. / d.as_secs_f32()))
//     }
//
//     fn get_max_update_duration(&self) -> Option<Duration> {
//         match self.limiter.limit_rate() {
//             None => None,
//             Some(update_rate) => Some(Duration::from_secs_f32(1. / update_rate)),
//         }
//     }
//
//     fn is_ups_over_max(&self) -> bool {
//         match (self.limiter.limit_rate(), self.get_updates_per_second()) {
//             (None, _) => false,
//             (_, None) => false,
//             (Some(max_ups), Some(ups)) => ups >= max_ups as f32,
//         }
//     }
// }

// impl crate::compute::PartialComputable for DiscreteTime {
//     fn compute_partial(&mut self, elapsed: Duration, until: impl Fn() -> bool) {
//         //
//         // let until_closure = {
//         //     let compute_timer = Instant::now();
//         //     move |quad: &Quad| {
//         //         until(self) || self.limiter.is_some_and(|d| d <= compute_timer.elapsed())}
//         //         //return bool to decide to stop or not (because of one compute constraint, or global update per second limit)
//         // };
//
//         //one update maximum :
//         // - if faster than required, lets rely on caller to call us again
//         // - if slower than needed, let's get out early.
//         self.world.compute_once_or_until(until_closure);
//
//         //if update was finished, increment timer
//         if self.world.is_updated() {
//             self.update_timer_tick();
//         }
//     }
//     fn update_completed(&self) -> bool {
//         self.world.is_updated()
//     }
// }
