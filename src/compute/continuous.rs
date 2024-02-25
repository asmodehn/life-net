use crate::compute::rate_limiter::RateLimiter;
use crate::graphics::view::Viewable;
use macroquad::texture::Image;
use std::time::{Duration, Instant};

use crate::compute::running_average::RunningAverage;
use crate::life::quad::Quad;

// //TODO : make it just a trait somehow ??
// struct ContinuousTime {
//     pub world: Quad, // TODO : replace with world (always continuous maybe ??)
//     average_duration: RunningAverage<Duration, N>,
//     limiter: RateLimiter,
// }
//
// impl Viewable for ContinuousTime {
//     fn render(&self) -> &Image {
//         // self.world.render()
//         // no need to render with a quad
//         &self.world.image
//     }
// }
