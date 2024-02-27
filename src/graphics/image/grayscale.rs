// use macroquad::color::Color;
// use macroquad::color_u8;
// use macroquad::math::Rect;
// use crate::graphics::color::monochrome::{ColorByte, Monochrome};
//
// pub(crate) struct Grayscale8 {
//     pub bytes: Vec<ColorByte>,
//     pub width: u16,
//     pub height: u16,
// }
//
// pub(crate) struct Grayscale32 {
//     pub bytes: Vec<Monochrome>,
//     pub width: u16,
//     pub height: u16,
// }

//
//
//
//
//
//
// impl Grayscale8 {
//     pub fn empty() -> Self{
//         Self {
//             width: 0,
//             height: 0,
//             bytes: vec![],
//         }
//     }
//
//     pub fn gen_grayscale(width: u16, height: u16, gray: u8) -> Grayscale8 {
//         let mut bytes: Vec<ColorByte> = vec![gray.into(); width as usize * height as usize];
//
//         Grayscale8 {
//             width,
//             height,
//             bytes,
//         }
//     }
//
//     /// Updates this image from a slice &[u8]s.
//     pub fn update(&mut self, gray_slice: &[u8]) {
//         assert_eq!(self.width as usize * self.height as usize, gray_slice.len());
//
//         for i in 0..gray_slice.len() {
//             self.bytes[i] = gray_slice[i].into();
//         }
//     }
//
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
//
//     /// Returns this image's data as a slice.
//     pub fn get_image_data(&self) -> &[u8] {
//         self.bytes.as_slice().into()
//     }
//
//     /// Returns this image's data as a mutable slice.
//     pub fn get_image_data_mut(&mut self) -> &mut [u8] {
//         self.bytes.as_mut_slice().into()
//     }
//
//     /// Modifies a pixel [Monochrome<u8>] in this image.
//     pub fn set_pixel(&mut self, x: u32, y: u32, gray: u8) {
//         let width = self.width;
//
//         self.get_image_data_mut()[(y * width as u32 + x) as usize] = gray;
//     }
//
//     /// Returns a pixel [Monochrome<u8>] from this image.
//     pub fn get_pixel(&self, x: u32, y: u32) -> u8 {
//         self.get_image_data()[(y * self.width as u32 + x) as usize].into()
//     }
//
//     /// Returns an Grayscale from a rect inside this image.
//     pub fn sub_image(&self, rect: Rect) -> Grayscale {
//         let width = rect.w as usize;
//         let height = rect.h as usize;
//         let mut bytes: Vec<Monochrome<u8>> = vec![Monochrome::<u8>::from(0u8); width * height];
//
//         let x = rect.x as usize;
//         let y = rect.y as usize;
//         let mut n = 0;
//         for y in y..y + height {
//             for x in x..x + width {
//                 bytes[n] = self.bytes[y * self.width as usize  + x ];
//                 n += 1;
//             }
//         }
//         Grayscale {
//             width: width as u16,
//             height: height as u16,
//             bytes,
//         }
//     }
// }
//
//
// #[cfg(test)]
// mod tests {
//     use test::Bencher;
//     use crate::graphics::color::monochrome::Monochrome;
//     use crate::graphics::image::grayscale::{Grayscale8, Grayscale32};
//
//     #[test]
//     fn check_empty_grayscale() {
//         let gs = Grayscale::empty();
//
//         assert_eq!(gs.width , 0);
//         assert_eq!(gs.height, 0);
//         assert!(gs.bytes.is_empty());
//
//         //tODO : assert all methods fail as expected...
//     }
//
//     #[test]
//     fn check_one_pixel_get() {
//
//         let gs = Grayscale::gen_grayscale(1, 1, 128u8);
//
//         assert_eq!(gs.get_pixel(0,0), 128u8);
//         assert_eq!(gs.get_image_data(), [128u8]);
//
//     }
//
//     #[test]
//     fn check_one_pixel_get_simplistic() {
//
//         let gs = Grayscale::gen_grayscale(1, 1, 128u8);
//
//         assert_eq!(gs.get_pixel(0,0), Monochrome::<u8>::from(128));
//         assert_eq!(gs.get_image_data(), [Monochrome::<u8>::from(128) ]);
//
//     }
//
//
//     #[test]
//     fn check_one_pixel_set_pixel() {
//
//         let mut gs = Grayscale::gen_grayscale(1, 1, 128u8.into());
//         gs.set_pixel(0,0,127u8.into());
//
//         assert_eq!(gs.get_pixel(0,0), Monochrome::<u8>::from(127u8));
//         assert_eq!(gs.get_image_data(), [Monochrome::<u8>::from(127u8) ]);
//     }
//
//     #[test]
//     fn check_one_pixel_set_data() {
//
//         let mut gs = Grayscale::gen_grayscale(1, 1, 128u8.into());
//         let mut d = gs.get_image_data_mut();
//         d[0] = 127u8.into();
//
//         assert_eq!(gs.get_pixel(0,0), Monochrome::<u8>::from(127u8));
//         assert_eq!(gs.get_image_data(), [Monochrome::<u8>::from(127u8) ]);
//     }
// }
