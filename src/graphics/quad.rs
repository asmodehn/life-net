// //immediate mode quad (with or without txture)
// //Note : A sprite has multiple textures, for animation....
//
// use macroquad::math::IVec2;
use macroquad::prelude::{draw_texture, Color, IVec3, UVec2};
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

trait Quad {
    fn dimensions(&self) -> UVec2;
    fn background(&self) -> Color;
}

//
// impl Displayable for Quad  {
//         fn display(&self, view: &View) {
//
//                 if self.texture.is_some() {
//                     draw_texture(&self.texture.unwrap(), self.position[0], self.position[1], self.color);
//                 } else {
//                     draw_rectangle(self.position[0], self.position[1], self.dimensions[0], self.dimensions[1], self.color);
//                 }
//             }
//         }
//     }
//
//
//
//
// }
//
//
//
