use itertools::Position;
// //immediate mode quad (with or without txture)
// //Note : A sprite has multiple textures, for animation....
//
// use macroquad::math::IVec2;
use macroquad::prelude::{draw_texture, Color, IVec2, IVec3, Image, UVec2};
// use macroquad::shapes::draw_rectangle;
// use macroquad::texture::Texture2D;
// use crate::graphics::{Displayable, Screen};
// use crate::life::cell::color;
// // use unique_id::sequence::SequenceGenerator;
// //
// // pub(crate) struct Quad {
// //     dimensions: UVec2,
// //     color: Color
// // }
//

pub(crate) trait Drawable {
    fn draw(&self, position_in_screen: IVec2);

    fn update(&mut self);
}

// pub(crate) trait Scalable {
//
//     fn scale(&mut self, factor: IVec2);
// }
//

pub(crate) trait Quad {
    // TODO : make it drawable (careful with views)
    fn get_dimensions(&self) -> UVec2;
    fn get_background(&self) -> Color; // TODO : move into drawable ??

    fn set_dimensions(&mut self, dimensions: UVec2);

    fn set_background(&mut self, color: Color);

    //TODO : make scale another trait (storing scale changes to keep original pixel-perfect)
    fn scale(&mut self, factor: u32);
}

pub trait Placed {
    fn get_position(&self) -> IVec2;

    fn set_position(&mut self, position: IVec2);

    fn translate(&mut self, displacement: IVec2);
}
