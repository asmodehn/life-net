use crate::graphics::quad::Drawable;
use crate::graphics::sprite::Sprite;
use crate::graphics::view::Viewable;
use macroquad::texture::Image;
use std::cell::RefMut;
use std::time::{Duration, Instant};

use crate::life::quad::Quad;
use crate::perf::DurationAverage;

//TODO : make it just a trait somehow ??
struct ContinuousTime<'w> {
    pub world: RefMut<'w, Quad>, // TODO : replace with world (always continuous maybe ??)
    average_duration: DurationAverage,
    drawable: Box<dyn Drawable>,
}

// impl Viewable for ContinuousTime {
//     fn render(&mut self) -> &Box<dyn Drawable> {
//         // self.world.render()
//         // no need to render with a quad
//         self.drawable = Box::new(Sprite::from_image(&self.world.image));
//         &self.drawable
//     }
// }

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
