mod gen;
mod grayscale;
mod rgb;

//
// use std::ffi::c_schar;
// use macroquad::math::Rect;
// use grayscale::Grayscale8;
// use rgb::RGBImage;
// use crate::graphics::color::monochrome::Monochrome;
// use crate::graphics::color::rgb::{RGB8, RGB32};
// use crate::graphics::image::grayscale::{Grayscale8, Grayscale32};
// use rgba::RGBAImage;
// use rgba::RGBA;

// impl From<Grayscale8> for RGB8 {
//     fn from(image: Grayscale8) -> Self {
//
//         let width = image.width;
//         let height = image.height;
//
//         let mut bytes: Vec<RGB8> = Vec::with_capacity((width * height) as usize);
//         let origin = image.get_image_data();
//
//         let mut n = 0;
//         for p in 0..origin.len() {
//             bytes[n] = RGB8::from([u8::from(origin[p]); 3]);
//             n += 1;
//         }
//
//         RGB8 {
//             0: 0,
//             1: 0,
//             width,
//             height,
//             bytes,
//             2: 0,
//         }
//     }
// }

#[cfg(test)]
mod tests {}
