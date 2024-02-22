use macroquad::color::YELLOW;
use macroquad::math::IVec2;
use std::cell::{Cell, Ref, RefCell};
use std::ops::{Add, Deref, MulAssign};
use std::rc::{Rc, Weak};
// use macroquad::color::Color;
// use macroquad::math::IVec2;
use macroquad::prelude::{draw_rectangle, draw_texture, Color, Image, Texture2D, UVec2};
// use crate::graphics::Displayable;
use crate::graphics::quad::{Drawable, Quad};
use crate::graphics::scene::Scene;
use crate::graphics::view::Viewable;
//

pub(crate) struct Sprite<V>
where
    V: Viewable,
{
    color: Color,
    dimensions: UVec2,
    texture: Option<Texture2D>, // NOT Copy !
    origin: Weak<Box<V>>,       // because Viewable may change
}

const DEFAULT_COLOR: Color = YELLOW;
const DEFAULT_DIMENSIONS: UVec2 = UVec2::new(16, 16);

impl<V: Viewable> Default for Sprite<V> {
    fn default() -> Self {
        Self {
            color: DEFAULT_COLOR,
            dimensions: DEFAULT_DIMENSIONS,
            texture: None,
            origin: Weak::<Box<V>>::new(),
        }
    }
}
impl<V: Viewable> Sprite<V> {
    pub(crate) fn new(color: Color, dimensions: UVec2) -> Self {
        Self {
            color,
            dimensions,
            ..Default::default()
        }
    }

    pub(crate) fn from_texture(texture2d: Texture2D) -> Self {
        Self {
            dimensions: UVec2::new(texture2d.width() as u32, texture2d.height() as u32),
            texture: Some(texture2d),
            ..Default::default()
        }
    }

    pub(crate) fn from_image(image: &Image) -> Self {
        Self {
            dimensions: UVec2::new(image.width() as u32, image.height() as u32),
            texture: Some(Texture2D::from_image(&image)),
            ..Default::default()
        }
    }

    pub(crate) fn from_viewable(viewable: Weak<Box<V>>) -> Self {
        let mut s = match viewable.upgrade() {
            None => Self::default(), // todo : or panic ?? return result ??
            Some(v) => Self::from_image(v.rendered()),
        };
        s.origin = viewable;
        s
    }
}

impl<V: Viewable> Drawable for Sprite<V> {
    fn draw(&self, position_in_view: IVec2) {
        if self.texture.is_some() {
            draw_texture(
                &self.texture.as_ref().unwrap(),
                position_in_view[0] as f32,
                position_in_view[1] as f32,
                self.color,
            );
        } else {
            draw_rectangle(
                position_in_view[0] as f32,
                position_in_view[1] as f32,
                self.dimensions[0] as f32,
                self.dimensions[1] as f32,
                self.color,
            );
        }
    }

    fn update(&mut self) {
        let origin = self.origin.upgrade();

        let image = match origin.as_ref() {
            None => None,
            Some(rc_bx_v) => Some(rc_bx_v.rendered()),
        };

        //TODO : what if none ??
        if image.is_some() {
            if self.texture.is_none() {
                self.texture = Some(Texture2D::from_image(image.unwrap()))
            } else {
                self.texture.as_mut().unwrap().update(image.unwrap())
            }
        }
    }
}

impl<V: Viewable> Quad for Sprite<V> {
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
