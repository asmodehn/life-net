use crate::graphics::color::RGBA8;
use crate::graphics::image::Image;
use macroquad::math::Rect;

pub(crate) struct RGBAImage {
    pub bytes: Vec<RGBA8>,
    pub width: u16,
    pub height: u16,
}

impl Image<u8, 4> for RGBAImage {
    type P = RGBA8;

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

    fn update(&mut self, pixel_slice: &[Self::P]) {
        assert_eq!(
            self.width as usize * self.height as usize,
            pixel_slice.len()
        );

        for i in 0..pixel_slice.len() {
            self.bytes[i] = pixel_slice[i];
        }
    }

    fn width(&self) -> usize {
        self.width as usize
    }

    fn height(&self) -> usize {
        self.height as usize
    }

    fn get_image_data(&self) -> &[Self::P] {
        self.bytes.as_slice()
    }

    fn get_image_data_mut(&mut self) -> &mut [Self::P] {
        self.bytes.as_mut_slice()
    }
    fn set_pixel(&mut self, x: u32, y: u32, pixel: Self::P) {
        let width = self.width;

        self.get_image_data_mut()[(y * width as u32 + x) as usize] = pixel;
    }

    fn get_pixel(&self, x: u32, y: u32) -> Self::P {
        self.get_image_data()[(y * self.width as u32 + x) as usize]
    }

    fn sub_image(&self, rect: Rect) -> Self {
        let width = rect.w as usize;
        let height = rect.h as usize;
        let mut bytes: Vec<Self::P> = vec![Self::P::from([0u8; 4]); width * height];

        let x = rect.x as usize;
        let y = rect.y as usize;
        let mut n = 0;
        for y in y..y + height {
            for x in x..x + width {
                bytes[n] = self.bytes[y * self.width as usize + x];
                // bytes[n + 1] = self.bytes[y * self.width as usize * 3 + x * 3 + 1];
                // bytes[n + 2] = self.bytes[y * self.width as usize * 3 + x * 3 + 2];
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
