use crate::graphics::color::monochrome::{ColorByte, Monochrome};
use crate::graphics::color::rgb::{RGB32, RGB8};

pub(crate) mod monochrome;
pub(crate) mod rgb;
pub(crate) mod rgba;

//todo : Custom ColorCode trait adding useful common functionality to all color types

trait Channel: Default + Copy + PartialEq {
    /// bits per component / color channel
    fn bpc() -> u32;
}

impl Channel for ColorByte {
    fn bpc() -> u32 {
        ColorByte::BITS
    }
}

impl Channel for Monochrome {
    fn bpc() -> u32 {
        32
    }
}

trait Pixel<C: Channel, const components: u8> {
    /// bits per pixel
    fn bpp() -> u32 {
        C::bpc() * components as u32
    }

    fn as_array(self) -> [C; components as usize]
    where
        Self: Sized,
    {
        todo!()
    }

    fn as_slice(&self) -> &[C; components as usize] {
        todo!()
    }
}

impl Pixel<ColorByte, 1> for ColorByte {
    fn as_array(self) -> [ColorByte; 1] {
        [self]
    }

    fn as_slice(&self) -> &[ColorByte; 1] {
        std::slice::from_ref(self).try_into().unwrap()
    }
}

impl Pixel<Monochrome, 1> for Monochrome {
    fn as_array(self) -> [Monochrome; 1] {
        todo!()
    }

    fn as_slice(&self) -> &[Monochrome; 1] {
        todo!()
    }
}

impl Pixel<ColorByte, 3> for RGB8 {
    fn as_array(self) -> [ColorByte; 3] {
        todo!()
    }

    fn as_slice(&self) -> &[ColorByte; 3] {
        todo!()
    }
}

impl Pixel<Monochrome, 3> for RGB32 {
    fn as_array(self) -> [Monochrome; 3] {
        todo!()
    }

    fn as_slice(&self) -> &[Monochrome; 3] {
        todo!()
    }
}

//TODO : conversion with Macroquad structs (Image, etc.)

#[cfg(test)]
mod tests {
    use test::Bencher;

    #[test]
    fn check_gray_u8_from_u8() {}
}
