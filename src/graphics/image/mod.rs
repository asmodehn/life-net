mod grayscale;
mod rgb;
mod rgba;

use crate::graphics::color::{Channel, Pixel};
use macroquad::math::Rect;

// Note : A trait is a safe way (avoids maintenance overhead) to keep implementations similar.
// But if they are *exactly* identical, then maybe we should have a generic struct instead ?
pub(crate) trait Image<C: Channel, const components: u8> {
    type P: Pixel<C, components>;

    fn empty() -> Self;

    fn generate(width: u16, height: u16, pixel: Self::P) -> Self;

    fn update(&mut self, pixel_slice: &[Self::P]);

    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn get_image_data(&self) -> &[Self::P];
    fn get_image_data_mut(&mut self) -> &mut [Self::P];
    fn set_pixel(&mut self, x: u32, y: u32, pixel: Self::P);

    fn get_pixel(&self, x: u32, y: u32) -> Self::P;
    fn sub_image(&self, rect: Rect) -> Self;
}

// TODO : in a separate texture module ?
// pub(crate) struct RGBTexture<T: Pixel<Monochrome, 3>> {
//     pub bytes: Vec<T>,
//     pub width: u16,
//     pub height: u16,
// }
//
// pub(crate) struct RGBATexture<T: Pixel<Monochrome, 4>> {
//     pub bytes: Vec<T>,
//     pub width: u16,
//     pub height: u16,
// }

//TODO : conversion with Macroquad structs (Image, etc.)

#[cfg(test)]
mod tests {}
