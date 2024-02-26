use crate::graphics::quad::{Drawable, Quad};
use macroquad::math::{IVec2, UVec2};
use macroquad::prelude::{draw_rectangle, Color};

struct Rect {
    color: Color,
    dimensions: UVec2,
}
//TODO : GLOBAL/Scene-projected position ??

impl Rect {
    pub fn new(color: Color, dimensions: UVec2) -> Self {
        Self { color, dimensions }
    }
}

impl Drawable for Rect {
    fn draw(&self, position: IVec2) {
        draw_rectangle(
            position[0] as f32,
            position[1] as f32,
            self.dimensions[0] as f32,
            self.dimensions[1] as f32,
            self.color,
        );
    }
}

impl Quad for Rect {
    fn get_dimensions(&self) -> UVec2 {
        self.dimensions
    }

    fn get_background(&self) -> Color {
        self.color
    }

    fn set_dimensions(&mut self, dimensions: UVec2) {
        self.dimensions = dimensions
    }

    fn set_background(&mut self, color: Color) {
        self.color = color
    }

    fn scale(&mut self, factor: u32) {
        self.dimensions.x *= factor;
        self.dimensions.y *= factor;
    }
}
