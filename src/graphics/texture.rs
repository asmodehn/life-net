use crate::graphics::quad::{Drawable, Quad};
use macroquad::math::{IVec2, UVec2};
use macroquad::prelude::{draw_texture, Color, FilterMode, Image, Texture2D};

struct Texture {
    color: Color,
    texture: Texture2D,
}

impl Texture {
    /// We always require an explicit background color, to help with debugging transparency issues.
    pub fn new(image: &Image, background: Color) -> Self {
        let t = Texture2D::from_image(image);
        t.set_filter(FilterMode::Nearest);

        Self {
            color: background,
            texture: t,
        }
    }
}

impl Drawable for Texture {
    fn draw(&self, position: IVec2) {
        draw_texture(
            &self.texture,
            position[0] as f32,
            position[1] as f32,
            self.color,
        );
    }
}

impl Quad for Texture {
    fn get_dimensions(&self) -> UVec2 {
        UVec2::new(self.texture.width() as u32, self.texture.height() as u32)
    }

    fn get_background(&self) -> Color {
        self.color
    }

    // NOT really changeable ??
    fn set_dimensions(&mut self, dimensions: UVec2) {
        //todo()
    }

    fn set_background(&mut self, color: Color) {
        self.color = color
    }

    // NOT freely scalable ??
    fn scale(&mut self, factor: u32) {
        //todo()
    }
}
