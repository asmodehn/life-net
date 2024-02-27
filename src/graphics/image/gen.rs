// use macroquad::math::Rect;
// use crate::graphics::color::grayscale::Gray;

pub(crate) struct Image<T: Default + Copy + PartialEq> {
    pub bytes: Vec<T>,
    pub width: u16,
    pub height: u16,
}
//
// impl<T: Default + Copy> Image<T> {
//     pub fn empty() -> Self{
//         Self {
//             width: 0,
//             height: 0,
//             bytes: vec![],
//         }
//     }
//
//     pub fn generate(width: u16, height: u16, gray: T) -> Self {
//         let mut bytes: Vec<T> = vec![gray; width as usize * height as usize];
//
//         Self{
//             width,
//             height,
//             bytes,
//         }
//     }
//
//     /// Updates this image from a slice &[u8]s.
//     pub fn update(&mut self, gray_slice: &[T]) {
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
//     pub fn get_image_data(&self) -> &[T] {
//         self.bytes.as_slice()
//     }
//
//     /// Returns this image's data as a mutable slice.
//     pub fn get_image_data_mut(&mut self) -> &mut [T] {
//         self.bytes.as_mut_slice()
//     }
//
//     /// Modifies a pixel [T] in this image.
//     pub fn set_pixel(&mut self, x: u32, y: u32, gray: T) {
//         let width = self.width;
//
//         self.get_image_data_mut()[(y * width as u32 + x) as usize] = gray;
//     }
//
//     /// Returns a pixel [T] from this image.
//     pub fn get_pixel(&self, x: u32, y: u32) -> T {
//         self.get_image_data()[(y * self.width as u32 + x) as usize]
//     }
//
//     /// Returns an Grayscale from a rect inside this image.
//     pub fn sub_image(&self, rect: Rect) -> Self {
//         let width = rect.w as usize;
//         let height = rect.h as usize;
//         let mut bytes: Vec<Gray<u8>> = vec![T::default(); width * height];
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
//         Self {
//             width: width as u16,
//             height: height as u16,
//             bytes,
//         }
//     }
// }
