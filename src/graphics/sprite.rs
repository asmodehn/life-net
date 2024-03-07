use crate::graphics::quad::{Drawable, Placed, Quad, Updatable};
use macroquad::color::YELLOW;
use macroquad::math::IVec2;
use macroquad::prelude::{draw_rectangle, draw_texture, Color, Image, Texture2D, UVec2};
use std::ops::AddAssign;

use crate::graphics::Viewable;
use delegate::delegate;

#[derive(Default)]
pub(crate) struct Sprite {
    color: Color,
    dimensions: UVec2,
    texture: Option<Texture2D>,
}
//TODO : a Vec of drawables instead ??

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
        let texture = Texture2D::from_image(image);

        Self {
            dimensions: UVec2::new(image.width() as u32, image.height() as u32),
            texture: Some(texture),
            ..Self::default()
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

impl Updatable for Sprite {
    fn update(&mut self, image: &Image) {
        if self.texture.is_none() {
            //we intentionally do not modify dimensions
            self.texture = Some(Texture2D::from_image(&image));
        } else {
            self.texture.as_mut().unwrap().update(image);
        }
    }
}

impl Quad for Sprite {
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

pub(crate) struct PlacedSprite {
    position: IVec2,
    sprite: Sprite,
}

const DEFAULT_POSITION: IVec2 = IVec2::new(0, 0);

impl PlacedSprite {
    pub fn new(sprite: Sprite) -> Self {
        Self {
            sprite,
            position: DEFAULT_POSITION,
        }
    }
}

impl Quad for PlacedSprite {
    delegate! {
        to self.sprite {
            fn get_dimensions(&self) -> UVec2;
            fn get_background(&self) -> Color;
            fn set_dimensions(&mut self, dimensions: UVec2);
            fn set_background(&mut self, color: Color);
            fn scale(&mut self, factor: u32);
        }
    }
}

impl Placed for PlacedSprite {
    fn get_position(&self) -> IVec2 {
        self.position
    }

    fn set_position(&mut self, position: IVec2) {
        self.position = position;
    }

    fn translate(&mut self, to: IVec2) {
        self.position.add_assign(to);
    }
}

impl Drawable for PlacedSprite {
    delegate! {
        to self.sprite {

    fn draw(&self, position_in_screen: IVec2);
        }
    }
}

impl Updatable for PlacedSprite {
    delegate! {
        to self.sprite {

    fn update(&mut self, image: &Image);
        }
    }
}
