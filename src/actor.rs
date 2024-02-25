use crate::graphics::quad::Drawable;
use crate::life::quad::Quad;
use macroquad::math::IVec2;
use macroquad::prelude::Image;
use std::time::{Duration, Instant};

pub(crate) trait Computable {
    fn compute(&mut self, elapsed: Duration, constraint: Option<Duration>);

    // TODO some kind of progress measurement ?
}

pub(crate) struct Actor<C, G> {
    pub(crate) graphics: G, // hopefully a simple graphics pipeline... pixel/rect focused.
    //TODO  : Vec of graphics components...
    // ..more stuff...
    pub(crate) compute: C, // where we will put all our ECS/capabilities for complex simulations later
}

impl<C: Computable, G: Drawable> Actor<C, G> {
    pub(crate) fn new(compute: C, graphics: G) -> Self {
        Self { compute, graphics }
    }
}

impl<C, G> Computable for Actor<C, G>
where
    C: Computable,
    G: Drawable,
{
    #[inline]
    fn compute(&mut self, elapsed: Duration, constraint: Option<Duration>) {
        self.compute.compute(elapsed, constraint)
    }
}

impl<C, G> Drawable for Actor<C, G>
where
    C: Computable,
    G: Drawable,
{
    #[inline]
    fn draw(&self, position_in_screen: IVec2) {
        self.graphics.draw(position_in_screen);
    }
}
