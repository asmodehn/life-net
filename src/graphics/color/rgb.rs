use crate::graphics::color::monochrome::{ColorByte, Monochrome};
use crate::graphics::color::Pixel;
use macroquad::math::{UVec3, Vec3};
use std::fmt::Debug;
use std::num::TryFromIntError;

// Newtype allowing for trait definitions (color encodig conversion f.i.)
/// RGB8 : RGB encoded with 8bit per component
#[repr(C)]
#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct RGB8([ColorByte; 3]);

//For relaxed Equality checks
// impl PartialEq<(ColorByte, ColorByte, ColorByte)> for RGB8 {
//     fn eq(&self, other: &(ColorByte, ColorByte, ColorByte)) -> bool {
//         self.0[0] == other.0[0] && self.0[1] == other.0[1] && self.0[2] == other.0[2]
//     }
// }

impl PartialEq<[ColorByte; 3]> for RGB8 {
    fn eq(&self, other: &[ColorByte; 3]) -> bool {
        self.0[0] == other[0] && self.0[1] == other[1] && self.0[2] == other[2]
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

//Arrays
impl From<RGB8> for [u8; 3] {
    #[inline]
    fn from(value: RGB8) -> Self {
        value.0
    }
}

impl From<[u8; 3]> for RGB8 {
    #[inline]
    fn from(value: [u8; 3]) -> Self {
        RGB8(value)
    }
}

impl From<RGB32> for [Monochrome; 3] {
    #[inline]
    fn from(value: RGB32) -> Self {
        [value.r.into(), value.g.into(), value.b.into()]
    }
}

impl From<[Monochrome; 3]> for RGB32 {
    #[inline]
    fn from(value: [Monochrome; 3]) -> Self {
        RGB32 {
            r: value[0],
            g: value[1],
            b: value[2],
        }
    }
}

//(U)Vec

impl From<RGB8> for UVec3 {
    #[inline]
    fn from(value: RGB8) -> Self {
        UVec3::from_array([value.0[0].into(), value.0[1].into(), value.0[2].into()])
    }
}

impl TryFrom<UVec3> for RGB8 {
    type Error = TryFromIntError;

    fn try_from(value: UVec3) -> Result<Self, Self::Error> {
        match (value.x.try_into(), value.y.try_into(), value.z.try_into()) {
            (Err(e), _, _) => Err(e),
            (_, Err(e), _) => Err(e),
            (_, _, Err(e)) => Err(e),
            (Ok(r), Ok(g), Ok(b)) => Ok(Self([r, g, b])),
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

#[cfg(test)]
mod tests {
    use crate::graphics::color::monochrome::Monochrome;
    use crate::graphics::color::rgb::{RGB32, RGB8};
    use macroquad::math::UVec3;
    use macroquad::prelude::Vec3;
    use test::Bencher;

    //From & Into Array

    #[test]
    fn check_rgb8_from_u8_array() {
        let c = RGB8::from([255u8, 255u8, 255u8]);
        assert_eq!(c, [255u8, 255u8, 255u8]);
    }

    #[test]
    fn check_u8_array_into_rgb8() {
        let c: RGB8 = [255u8, 255u8, 255u8].into();
        assert_eq!(c, [255u8, 255u8, 255u8]);
    }

    #[test]
    fn check_u8_array_from_rgb8() {
        let c = <[u8; 3]>::from(RGB8([255u8, 255u8, 255u8]));
        assert_eq!(c, [255u8, 255u8, 255u8]);
    }

    #[test]
    fn check_rgb8_into_u8_array() {
        let c: [u8; 3] = RGB8([255u8, 255u8, 255u8]).into();
        assert_eq!(c, [255u8, 255u8, 255u8]);
    }

    #[test]
    fn check_rgb32_from_monochrome_array() {
        let c = RGB32::from([
            Monochrome::try_from(1f32).unwrap(),
            Monochrome::try_from(1f32).unwrap(),
            Monochrome::try_from(1f32).unwrap(),
        ]);
        assert_eq!(c.r, 1.);
        assert_eq!(c.g, 1.);
        assert_eq!(c.b, 1.);
    }
    #[test]
    fn check_monochrome_array_into_rgb32() {
        let c: RGB32 = [
            Monochrome::try_from(1f32).unwrap(),
            Monochrome::try_from(1f32).unwrap(),
            Monochrome::try_from(1f32).unwrap(),
        ]
        .into();
        assert_eq!(c.r, 1.);
        assert_eq!(c.g, 1.);
        assert_eq!(c.b, 1.);
    }

    #[test]
    fn check_monochrome_array_from_rgb32() {
        let c = <[Monochrome; 3]>::from(RGB32 {
            r: 1f32.try_into().unwrap(),
            g: 1f32.try_into().unwrap(),
            b: 1f32.try_into().unwrap(),
        });
        assert_eq!(
            c,
            [
                Monochrome::try_from(1.).unwrap(),
                Monochrome::try_from(1.).unwrap(),
                Monochrome::try_from(1.).unwrap()
            ]
        );
    }

    #[test]
    fn check_rgb32_into_monochrome_array() {
        let c: [Monochrome; 3] = RGB32 {
            r: 1f32.try_into().unwrap(),
            g: 1f32.try_into().unwrap(),
            b: 1f32.try_into().unwrap(),
        }
        .into();
        assert_eq!(
            c,
            [
                Monochrome::try_from(1.).unwrap(),
                Monochrome::try_from(1.).unwrap(),
                Monochrome::try_from(1.).unwrap()
            ]
        );
    }

    //from & into ref ??

    // #[test]
    // fn check_rgb8_ref_from_u8_array_ref() {
    //     let a = [255u8, 255u8, 255u8];
    //     let c: &RGB8 = (&a).into();
    //     assert_eq!(*c, [255u8, 255u8, 255u8]);
    // }

    //From & Into (U)Vec
    #[test]
    fn check_rgb8_from_uvec3() {
        let c = RGB8::try_from(UVec3::new(255, 255, 255)).unwrap();
        assert_eq!(c, [255u8, 255u8, 255u8]);
    }
    #[test]
    fn check_uvec3_into_rgb8() {
        let c: RGB8 = UVec3::new(255, 255, 255).try_into().unwrap();
        assert_eq!(c, [255u8, 255u8, 255u8]);
    }

    #[test]
    fn check_rgb32_from_vec3() {
        let c = RGB32::try_from(Vec3::new(1., 1., 1.)).unwrap();
        assert_eq!(c.r, 1.);
        assert_eq!(c.g, 1.);
        assert_eq!(c.b, 1.);
    }
    #[test]
    fn check_vec3_into_rgb32() {
        let c: RGB32 = Vec3::new(1., 1., 1.).try_into().unwrap();
        assert_eq!(c.r, 1.);
        assert_eq!(c.g, 1.);
        assert_eq!(c.b, 1.);
    }
}
