use crate::compute::Computable;

pub(crate) mod monochrome;
pub use monochrome::{ColorByte, Monochrome};

pub(crate) mod rgb;
pub(crate) mod rgba;

pub use rgb::{RGB32, RGB8};
pub use rgba::{RGBA32, RGBA8};

pub(crate) trait Channel: Default + Copy + PartialEq {
    /// bits per component / color channel
    fn bpc() -> u32;
}

impl const Channel for ColorByte {
    fn bpc() -> u32 {
        ColorByte::BITS
    }
}

impl const Channel for Monochrome {
    fn bpc() -> u32 {
        32
    }
}

pub(crate) trait Pixel<C: Channel, const components: u8>:
    Default + Copy + PartialEq
{
    /// bits per pixel
    fn bpp() -> u32 {
        C::bpc() * components as u32
    }

    // fn as_array(self) -> [C; components as usize]
    // where Self: Sized;
}

impl Pixel<ColorByte, 1> for ColorByte {
    // fn as_array(self) -> [ColorByte; 1] {
    //     [self]
    // }
}

impl Pixel<Monochrome, 1> for Monochrome {
    // fn as_array(self) -> [Monochrome; 1] {
    //     [self]
    // }
}

impl Pixel<ColorByte, 3> for RGB8 {
    // fn as_array(self) -> [ColorByte; 3] {
    //     self.into()
    // }
}

impl Pixel<Monochrome, 3> for RGB32 {
    // fn as_array(self) -> [Monochrome; 3] {
    //     self.into()
    // }
}

impl Pixel<ColorByte, 4> for RGBA8 {
    // fn as_array(self) -> [ColorByte; 4] {
    //     self.into()
    // }
}

impl Pixel<Monochrome, 4> for RGBA32 {
    // fn as_array(self) -> [Monochrome; 4] {
    //     self.into()
    // }
}

#[cfg(test)]
mod tests {
    use test::Bencher;

    #[test]
    fn check_gray_u8_from_u8() {}
}
