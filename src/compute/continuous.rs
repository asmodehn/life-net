use crate::graphics::view::Viewable;
use macroquad::texture::Image;
use std::time::{Duration, Instant};

use crate::life::quad::Quad;
use crate::perf::RunningAverage;

//TODO : make it just a trait somehow ??
struct ContinuousTime {
    pub world: Quad, // TODO : replace with world (always continuous maybe ??)
    average_duration: RunningAverage<Duration>,
}

impl Viewable for ContinuousTime {
    fn render(&self) -> &Image {
        // self.world.render()
        // no need to render with a quad
        &self.world.image
    }
}

// NOT FOR NOW : keep it just a trait...
// impl SimulationTrait for ContinuousTime {
//     fn update(&mut self, elapsed: Duration, constraint: Duration) {
//
//         //total update
//         let update_timer= Some(Instant::now());
//         self.world.update(elapsed, constraint, |quad: &Quad| quad.completed());
//
//         self.average_duration.record(update_timer?.elapsed())
//     }
//
//     fn get_updates_per_second(&self) -> Option<f32> {
//         self.average_duration.per_second()
//     }
// }
