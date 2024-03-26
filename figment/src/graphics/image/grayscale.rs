use macroquad::math::Rect;
// use macroquad::color::Color;
// use macroquad::color_u8;
// use macroquad::math::Rect;
// use crate::graphics::color::monochrome::{ColorByte, Monochrome};
//
use crate::graphics::image::rgb::RGBImage;
use crate::graphics::image::Image;

pub(crate) struct GrayscaleImage {
    pub bytes: Vec<u8>,
    pub width: u16,
    pub height: u16,
}

impl Image<u8, 1> for GrayscaleImage {
    type P = u8;

    fn empty() -> Self {
        Self {
            width: 0,
            height: 0,
            bytes: vec![],
        }
    }

    fn generate(width: u16, height: u16, pixel: Self::P) -> Self {
        let bytes: Vec<Self::P> = vec![pixel; width as usize * height as usize];

        Self {
            width,
            height,
            bytes,
        }
    }

    fn update(&mut self, gray_slice: &[Self::P]) {
        assert_eq!(self.width as usize * self.height as usize, gray_slice.len());

        for i in 0..gray_slice.len() {
            self.bytes[i] = gray_slice[i];
        }
    }

    fn width(&self) -> usize {
        self.width as usize
    }

    fn height(&self) -> usize {
        self.height as usize
    }

    fn get_image_data(&self) -> &[Self::P] {
        self.bytes.as_slice().into()
    }

    fn get_image_data_mut(&mut self) -> &mut [Self::P] {
        self.bytes.as_mut_slice().into()
    }

    fn set_pixel(&mut self, x: u32, y: u32, pixel: Self::P) {
        let width = self.width;

        self.get_image_data_mut()[(y * width as u32 + x) as usize] = pixel;
    }

    fn get_pixel(&self, x: u32, y: u32) -> Self::P {
        self.get_image_data()[(y * self.width as u32 + x) as usize].into()
    }

    fn sub_image(&self, rect: Rect) -> Self {
        let width = rect.w as usize;
        let height = rect.h as usize;
        let mut bytes: Vec<u8> = vec![0u8; width * height];

        let x = rect.x as usize;
        let y = rect.y as usize;
        let mut n = 0;
        for y in y..y + height {
            for x in x..x + width {
                bytes[n] = self.bytes[y * self.width as usize + x];
                n += 1;
            }
        }
        Self {
            width: width as u16,
            height: height as u16,
            bytes,
        }
    }
}

//
// pub(crate) struct Grayscale32 {
//     pub bytes: Vec<Monochrome>,
//     pub width: u16,
//     pub height: u16,
// }

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
