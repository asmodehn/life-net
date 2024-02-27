use crate::graphics::color::monochrome::{ColorByte, Monochrome};
use macroquad::color::Color;
use macroquad::math::{UVec3, Vec3};
use std::fmt::Debug;
use std::num::TryFromIntError;
// use crate::graphics::color::traits::{FromColor, IntoColor};

// Newtype allowing for trait definitions
/// RGB8 : RGB encoded with 8bit per component
#[repr(C)]
#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct RGB8(ColorByte, ColorByte, ColorByte);

//For relaxed Equality checks
impl PartialEq<(ColorByte, ColorByte, ColorByte)> for RGB8 {
    fn eq(&self, other: &(ColorByte, ColorByte, ColorByte)) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}

/// RGB32 : RGB encoded with 32bit per component
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RGB32 {
    pub r: Monochrome,
    pub g: Monochrome,
    pub b: Monochrome,
}

impl Default for RGB32 {
    fn default() -> Self {
        Self {
            r: Monochrome::default(),
            g: Monochrome::default(),
            b: Monochrome::default(),
        }
    }
}

//IMPLS From reflexively

//(U)Vec

impl From<RGB8> for UVec3 {
    #[inline]
    fn from(value: RGB8) -> Self {
        UVec3::new(value.0.into(), value.1.into(), value.2.into())
    }
}

impl TryFrom<UVec3> for RGB8 {
    type Error = TryFromIntError;

    fn try_from(value: UVec3) -> Result<Self, Self::Error> {
        match (value.x.try_into(), value.y.try_into(), value.z.try_into()) {
            (Err(e), _, _) => Err(e),
            (_, Err(e), _) => Err(e),
            (_, _, Err(e)) => Err(e),
            (Ok(r), Ok(g), Ok(b)) => Ok(Self(r, g, b)),
        }
    }
}

impl From<RGB32> for Vec3 {
    #[inline]
    fn from(value: RGB32) -> Self {
        Vec3::new(value.r.into(), value.g.into(), value.b.into())
    }
}

impl TryFrom<Vec3> for RGB32 {
    type Error = String;

    fn try_from(value: Vec3) -> Result<Self, Self::Error> {
        match (value.x.try_into(), value.y.try_into(), value.z.try_into()) {
            (Err(e), _, _) => Err(e),
            (_, Err(e), _) => Err(e),
            (_, _, Err(e)) => Err(e),
            (Ok(r), Ok(g), Ok(b)) => Ok(Self { r, g, b }),
        }
    }
}

//Arrays
//
// impl<T> From<RGB<T>> for [u8;3]
//     where T:  Copy + Debug + PartialEq
// {
//     #[inline]
//     fn from(value: RGB<T>) -> Self {
//         [
//             T::into_color_value(value.r),
//             T::into_color_value(value.g),
//             T::into_color_value(value.b)
//         ]
//     }
// }
//
// impl<T> From<[u8;3]> for RGB<T>
//     where T:  Copy + Debug + PartialEq
// {
//     #[inline]
//     fn from(value: [u8;3]) -> Self {
//         Self{ r: T::from_color_value(value[0]),
//             g: T::from_color_value(value[1]),
//             b: T::from_color_value(value[2]) }
//     }
// }
//
// impl<T> From<RGB<T>> for [f32;3]
//     where T:  Copy + Debug + PartialEq {
//     #[inline]
//     fn from(value: RGB<T>) -> Self {
//         [
//             T::into_color_value(value.r),
//
//             T::into_color_value(value.g),
//
//             T::into_color_value(value.b)
//         ]
//     }
// }
//
// impl<T> From<[f32;3]> for RGB<T>
//     where T:  Copy + Debug + PartialEq
// {
//     #[inline]
//     fn from(value: [f32;3]) -> Self {
//         Self{
//             r: T::from_color_value(value[0]),
//             g: T::from_color_value(value[1]),
//             b: T::from_color_value(value[2])}
//     }
// }

//TODO : slice / mut slices ??

#[cfg(test)]
mod tests {
    use crate::graphics::color::rgb::{RGB32, RGB8};
    use macroquad::math::UVec3;
    use macroquad::prelude::Vec3;
    use test::Bencher;

    //From & Into (U)Vec
    #[test]
    fn check_rgb_u8_from_uvec3() {
        let c = RGB8::try_from(UVec3::new(255, 255, 255)).unwrap();
        assert_eq!(c, (255u8, 255u8, 255u8));
    }
    #[test]
    fn check_uvec3_into_rgb_u8() {
        let c: RGB8 = UVec3::new(255, 255, 255).try_into().unwrap();
        assert_eq!(c, (255u8, 255u8, 255u8));
    }

    #[test]
    fn check_rgb_f32_from_vec3() {
        let c = RGB32::try_from(Vec3::new(1., 1., 1.)).unwrap();
        assert_eq!(c.r, 1.);
        assert_eq!(c.g, 1.);
        assert_eq!(c.b, 1.);
    }
    #[test]
    fn check_vec3_into_rgb_f32() {
        let c: RGB32 = Vec3::new(1., 1., 1.).try_into().unwrap();
        assert_eq!(c.r, 1.);
        assert_eq!(c.g, 1.);
        assert_eq!(c.b, 1.);
    }
    //
    // #[test]
    // fn check_rgb_f32_from_uvec3() {
    //
    //     let c = RGB32::from(UVec3::new(255, 255, 255));
    //     assert_eq!(c.r, 1.);
    //     assert_eq!(c.g, 1.);
    //     assert_eq!(c.b, 1.);
    // }
    // #[test]
    // fn check_uvec3_into_rgb_f32() {
    //     let c: RGB<f32> = UVec3::new(255,255,255).into();
    //     assert_eq!(c.r, 1.);
    //     assert_eq!(c.g, 1.);
    //     assert_eq!(c.b, 1.);
    // }
    //
    //
    // #[test]
    // fn check_rgb_u8_from_vec3() {
    //     let c = RGB::<u8>::from(Vec3::new(1., 1., 1.));
    //     assert_eq!(c.r, 255);
    //     assert_eq!(c.g, 255);
    //     assert_eq!(c.b, 255);
    //
    // }
    // #[test]
    // fn check_vec3_into_rgb_u8() {
    //     let c: RGB<u8> = Vec3::new(1.,1.,1.).into();
    //     assert_eq!(c.r, 255);
    //     assert_eq!(c.g, 255);
    //     assert_eq!(c.b, 255);
    // }

    //From & Into Array

    // #[test]
    // fn check_rgb_u8_from_u8_array() {
    //
    //     let c = RGB::<u8>::from([255u8, 255u8, 255u8]);
    //     assert_eq!(c.r, 255);
    //     assert_eq!(c.g, 255);
    //     assert_eq!(c.b, 255);
    // }
    // #[test]
    // fn check_u8_array_into_rgb_u8() {
    //     let c: RGB<u8> = [255u8, 255u8, 255u8].into();
    //     assert_eq!(c.r, 255);
    //     assert_eq!(c.g, 255);
    //     assert_eq!(c.b, 255);
    // }
    //
    //
    // #[test]
    // fn check_rgb_f32_from_f32_array() {
    //
    //     let c = RGB::<f32>::from([1., 1., 1.]);
    //     assert_eq!(c.r, 1.);
    //     assert_eq!(c.g, 1.);
    //     assert_eq!(c.b, 1.);
    //
    // }
    // #[test]
    // fn check_f32_array_into_rgb_f32() {
    //     let c: RGB<f32> = [1., 1., 1.].into();
    //     assert_eq!(c.r, 1.);
    //     assert_eq!(c.g, 1.);
    //     assert_eq!(c.b, 1.);
    // }
    //
    // #[test]
    // fn check_rgb_f32_from_u8_array() {
    //
    //     let c = RGB::<f32>::from([255u8, 255u8, 255u8]);
    //     assert_eq!(c.r, 1.);
    //     assert_eq!(c.g, 1.);
    //     assert_eq!(c.b, 1.);
    // }
    // #[test]
    // fn check_u8_array_into_rgb_f32() {
    //     let c: RGB<f32> = [255u8, 255u8, 255u8].into();
    //     assert_eq!(c.r, 1.);
    //     assert_eq!(c.g, 1.);
    //     assert_eq!(c.b, 1.);
    // }
    //
    //
    // #[test]
    // fn check_rgb_u8_from_f32_array() {
    //     let c = RGB::<u8>::from([1., 1., 1.]);
    //     assert_eq!(c.r, 255);
    //     assert_eq!(c.g, 255);
    //     assert_eq!(c.b, 255);
    //
    // }
    // #[test]
    // fn check_f32_array_into_rgb_u8() {
    //     let c: RGB<u8> = [1., 1., 1.].into();
    //     assert_eq!(c.r, 255);
    //     assert_eq!(c.g, 255);
    //     assert_eq!(c.b, 255);
    // }
}
