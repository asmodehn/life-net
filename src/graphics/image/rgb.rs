// use core::slice::SlicePattern;
// use macroquad::color::Color;
// use macroquad::math::Rect;
// use macroquad::prelude::Image;
// pub use crate::graphics::color::rgb::RGB;

// pub(crate) struct RGB8Image {
//     pub bytes: Vec<RGB8>,
//     pub width: u16,
//     pub height: u16,
// }
//
//
// impl RGBImage {
//
//     /// Creates an empty opaque Image.
//     pub fn empty() -> Image {
//         Image {
//             width: 0,
//             height: 0,
//             bytes: vec![],
//         }
//     }
//
//     /// Creates an Image filled with the provided [Color].
//     pub fn gen_image_color(width: u16, height: u16, color: RGB<u8>) -> RGBImage {
//         let mut bytes: Vec<RGB<u8>> = vec![color; width as usize * height as usize * 3];
//         RGBImage {
//             width,
//             height,
//             bytes,
//         }
//     }
//
//     /// Updates this image from a slice of [Color]s.
//     pub fn update(&mut self, colors: &[RGB<u8>]) {
//         assert_eq!(self.width as usize * self.height as usize, colors.len());
//
//         for i in 0..colors.len() {
//             self.bytes[i] = colors[i];
//         }
//     }
//
//     /// Returns the width of this image.
//     pub fn width(&self) -> usize {
//         self.width as usize
//     }
//
//     /// Returns the height of this image.
//     pub fn height(&self) -> usize {
//         self.height as usize
//     }
//
//     /// Returns this image's data as a slice of 4-Byte arrays.
//     pub fn get_image_data(&self) -> &[RGB<u8>] {
//
//         self.bytes.as_slice()
//
//         // use std::slice;
//         //
//         // unsafe {
//         //     slice::from_raw_parts(
//         //         self.bytes.as_ptr() as *const [u8; 3],
//         //         self.width as usize * self.height as usize,
//         //     )
//         // }
//     }
//
//     /// Returns this image's data as a mutable slice of 4-Byte arrays.
//     pub fn get_image_data_mut(&mut self) -> &mut [RGB<u8>] {
//         self.bytes.as_mut_slice()
//         // use std::slice;
//         //
//         // unsafe {
//         //     slice::from_raw_parts_mut(
//         //         self.bytes.as_mut_ptr() as *mut [u8; 3],
//         //         self.width as usize * self.height as usize,
//         //     )
//         // }
//     }
//
//     /// Modifies a pixel [Color] in this image.
//     pub fn set_pixel(&mut self, x: u32, y: u32, color: RGB<u8>) {
//         let width = self.width;
//
//         self.get_image_data_mut()[(y * width as u32 + x) as usize] = color;
//     }
//
//     /// Returns a pixel [Color] from this image.
//     pub fn get_pixel(&self, x: u32, y: u32) ->RGB<u8> {
//         self.get_image_data()[(y * self.width as u32 + x) as usize]
//     }
//
//     /// Returns an Image from a rect inside this image.
//     pub fn sub_image(&self, rect: Rect) -> RGBImage {
//         let width = rect.w as usize;
//         let height = rect.h as usize;
//         let mut bytes: Vec<RGB<u8>> = vec![RGB::from([0u8, 0u8, 0u8]); width * height * 3];
//
//         let x = rect.x as usize;
//         let y = rect.y as usize;
//         let mut n = 0;
//         for y in y..y + height {
//             for x in x..x + width {
//                 bytes[n] = self.bytes[y * self.width as usize + x ];
//                 // bytes[n + 1] = self.bytes[y * self.width as usize * 3 + x * 3 + 1];
//                 // bytes[n + 2] = self.bytes[y * self.width as usize * 3 + x * 3 + 2];
//                 n += 1;
//             }
//         }
//         RGBImage {
//             width: width as u16,
//             height: height as u16,
//             bytes,
//         }
//     }

// }
