use macroquad::color::YELLOW;
use macroquad::math::IVec2;
// use macroquad::color::Color;
// use macroquad::math::IVec2;
use macroquad::prelude::{draw_rectangle, draw_texture, Color, Image, Texture2D, UVec2};
// use crate::graphics::Displayable;
// use crate::graphics::quad::Quad;
//

pub(crate) trait Drawable {
    fn draw(&self, position: IVec2);
}

#[derive(Default)]
pub(crate) struct Sprite {
    color: Color,
    dimensions: UVec2,
    texture: Option<Texture2D>,
}

const DEFAULT_COLOR: Color = YELLOW;
const DEFAULT_DIMENSIONS: UVec2 = UVec2::new(16, 16);

impl Sprite {
    fn default() -> Self {
        Self {
            color: DEFAULT_COLOR,
            dimensions: DEFAULT_DIMENSIONS,
            texture: None,
        }
    }

    fn new(texture2d: Texture2D) -> Self {
        Self {
            dimensions: UVec2::new(texture2d.width() as u32, texture2d.height() as u32),
            texture: Some(texture2d),
            ..Self::default()
        }
    }

    pub(crate) fn from_image(image: &Image) -> Self {
        Self {
            dimensions: UVec2::new(image.width() as u32, image.height() as u32),
            texture: Some(Texture2D::from_image(&image)),
            ..Self::default()
        }
    }

    pub(crate) fn update(&mut self, image: &Image) {
        if self.texture.is_none() {
            //we do not modify dimensions : intended.
            self.texture = Some(Texture2D::from_image(&image));
        } else {
            self.texture.as_mut().unwrap().update(image);
        }
    }
}

impl Drawable for Sprite {
    fn draw(&self, position: IVec2) {
        if self.texture.is_some() {
            draw_texture(
                &self.texture.as_ref().unwrap(),
                position[0] as f32,
                position[1] as f32,
                self.color,
            );
        } else {
            draw_rectangle(
                position[0] as f32,
                position[1] as f32,
                self.dimensions[0] as f32,
                self.dimensions[1] as f32,
                self.color,
            );
        }
    }
}
