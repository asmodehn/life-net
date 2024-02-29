use crate::graphics::color::monochrome::{ColorByte, Monochrome};
use macroquad::math::{UVec4, Vec4};
use std::fmt::Debug;
use std::num::TryFromIntError;

// use crate::graphics::color::traits::{ FromColor, IntoColor};

//type alias because there are the *same*
pub type RGBA8Hex = u32;

// Newtype allowing for trait definitions
/// RGB8 : RGB encoded with 8bit per component
#[repr(C)]
#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct RGBA8([ColorByte; 4]);

//For relaxed Equality checks
impl PartialEq<[ColorByte; 4]> for RGBA8 {
    fn eq(&self, other: &[ColorByte; 4]) -> bool {
        self.0[0] == other[0]
            && self.0[1] == other[1]
            && self.0[2] == other[2]
            && self.0[3] == other[3]
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RGBA32 {
    pub r: Monochrome,
    pub g: Monochrome,
    pub b: Monochrome,
    pub a: Monochrome,
}

impl Default for RGBA32 {
    fn default() -> Self {
        Self {
            r: Monochrome::default(),
            g: Monochrome::default(),
            b: Monochrome::default(),
            a: Monochrome::default(),
        }
    }
}

//IMPLS From reflexively

//Arrays

impl From<RGBA8> for [u8; 4] {
    #[inline]
    fn from(value: RGBA8) -> Self {
        value.0
    }
}

impl From<[u8; 4]> for RGBA8 {
    #[inline]
    fn from(value: [u8; 4]) -> Self {
        RGBA8(value)
    }
}

impl From<RGBA32> for [Monochrome; 4] {
    #[inline]
    fn from(value: RGBA32) -> Self {
        [
            value.r.into(),
            value.g.into(),
            value.b.into(),
            value.a.into(),
        ]
    }
}

impl From<[Monochrome; 4]> for RGBA32 {
    #[inline]
    fn from(value: [Monochrome; 4]) -> Self {
        Self {
            r: value[0],
            g: value[1],
            b: value[2],
            a: value[3],
        }
    }
}

//(U)Vec

impl From<RGBA8> for UVec4 {
    #[inline]
    fn from(value: RGBA8) -> Self {
        UVec4::new(
            value.0[0].into(),
            value.0[1].into(),
            value.0[2].into(),
            value.0[3].into(),
        )
    }
}

impl TryFrom<UVec4> for RGBA8 {
    type Error = TryFromIntError;

    fn try_from(value: UVec4) -> Result<Self, Self::Error> {
        match (
            value.x.try_into(),
            value.y.try_into(),
            value.z.try_into(),
            value.w.try_into(),
        ) {
            (Err(e), _, _, _) => Err(e),
            (_, Err(e), _, _) => Err(e),
            (_, _, Err(e), _) => Err(e),
            (_, _, _, Err(e)) => Err(e),
            (Ok(r), Ok(g), Ok(b), Ok(a)) => Ok(Self([r, g, b, a])),
        }
    }
}

impl From<RGBA32> for Vec4 {
    #[inline]
    fn from(value: RGBA32) -> Self {
        Vec4::new(
            value.r.into(),
            value.g.into(),
            value.b.into(),
            value.a.into(),
        )
    }
}

impl TryFrom<Vec4> for RGBA32 {
    type Error = String;

    fn try_from(value: Vec4) -> Result<Self, Self::Error> {
        match (
            value.x.try_into(),
            value.y.try_into(),
            value.z.try_into(),
            value.w.try_into(),
        ) {
            (Err(e), _, _, _) => Err(e),
            (_, Err(e), _, _) => Err(e),
            (_, _, Err(e), _) => Err(e),
            (_, _, _, Err(e)) => Err(e),
            (Ok(r), Ok(g), Ok(b), Ok(a)) => Ok(Self { r, g, b, a }),
        }
    }
}

//TODO : ref/ mut ref ??

#[cfg(test)]
mod tests {
    use crate::graphics::color::monochrome::Monochrome;
    use crate::graphics::color::rgb::RGB32;
    use crate::graphics::color::rgba::{RGBA32, RGBA8};
    use macroquad::math::UVec4;
    use macroquad::prelude::Vec4;
    use test::Bencher;

    //From & Into Array

    #[test]
    fn check_rgba8_from_u8_array() {
        let c = RGBA8::from([255u8, 255u8, 255u8, 255u8]);
        assert_eq!(c, [255u8, 255u8, 255u8, 255u8]);
    }
    #[test]
    fn check_u8_array_into_rgba8() {
        let c: RGBA8 = [255u8, 255u8, 255u8, 255u8].into();
        assert_eq!(c, [255u8, 255u8, 255u8, 255u8]);
    }

    #[test]
    fn check_u8_array_from_rgba8() {
        let c = <[u8; 4]>::from(RGBA8([255u8, 255u8, 255u8, 255u8]));
        assert_eq!(c, [255u8, 255u8, 255u8, 255u8]);
    }
    #[test]
    fn check_rgba8_into_u8_array() {
        let c: [u8; 4] = RGBA8([255u8, 255u8, 255u8, 255u8]).into();
        assert_eq!(c, [255u8, 255u8, 255u8, 255u8]);
    }

    #[test]
    fn check_rgba_f32_from_f32_array() {
        let c = RGBA32::from([
            Monochrome::try_from(1f32).unwrap(),
            Monochrome::try_from(1f32).unwrap(),
            Monochrome::try_from(1f32).unwrap(),
            Monochrome::try_from(1f32).unwrap(),
        ]);
        assert_eq!(c.r, 1.);
        assert_eq!(c.g, 1.);
        assert_eq!(c.b, 1.);
        assert_eq!(c.a, 1.);
    }
    #[test]
    fn check_f32_array_into_rgba_f32() {
        let c: RGBA32 = [
            Monochrome::try_from(1f32).unwrap(),
            Monochrome::try_from(1f32).unwrap(),
            Monochrome::try_from(1f32).unwrap(),
            Monochrome::try_from(1f32).unwrap(),
        ]
        .into();
        assert_eq!(c.r, 1.);
        assert_eq!(c.g, 1.);
        assert_eq!(c.b, 1.);
        assert_eq!(c.a, 1.);
    }

    #[test]
    fn check_f32_array_from_rgba_f32() {
        let c = <[Monochrome; 4]>::from(RGBA32 {
            r: 1f32.try_into().unwrap(),
            g: 1f32.try_into().unwrap(),
            b: 1f32.try_into().unwrap(),
            a: 1f32.try_into().unwrap(),
        });
        assert_eq!(
            c,
            [
                Monochrome::try_from(1.).unwrap(),
                Monochrome::try_from(1.).unwrap(),
                Monochrome::try_from(1.).unwrap(),
                Monochrome::try_from(1.).unwrap()
            ]
        );
    }

    #[test]
    fn check_rgba_f32_into_f32_array() {
        let c: [Monochrome; 4] = RGBA32 {
            r: 1f32.try_into().unwrap(),
            g: 1f32.try_into().unwrap(),
            b: 1f32.try_into().unwrap(),
            a: 1f32.try_into().unwrap(),
        }
        .into();
        assert_eq!(
            c,
            [
                Monochrome::try_from(1.).unwrap(),
                Monochrome::try_from(1.).unwrap(),
                Monochrome::try_from(1.).unwrap(),
                Monochrome::try_from(1.).unwrap()
            ]
        );
    }

    //from & into ref ??

    //From & Into (U)Vec
    #[test]
    fn check_rgba_u8_from_uvec4() {
        let c = RGBA8::try_from(UVec4::new(255, 255, 255, 255)).unwrap();
        assert_eq!(c, [255u8, 255u8, 255u8, 255u8]);
    }
    #[test]
    fn check_uvec4_into_rgba_u8() {
        let c: RGBA8 = UVec4::new(255, 255, 255, 255).try_into().unwrap();
        assert_eq!(c, [255u8, 255u8, 255u8, 255u8]);
    }

    #[test]
    fn check_rgba_f32_from_vec4() {
        let c = RGBA32::try_from(Vec4::new(1., 1., 1., 1.)).unwrap();
        assert_eq!(c.r, 1.);
        assert_eq!(c.g, 1.);
        assert_eq!(c.b, 1.);
        assert_eq!(c.a, 1.);
    }
    #[test]
    fn check_vec4_into_rgba_f32() {
        let c: RGBA32 = Vec4::new(1., 1., 1., 1.).try_into().unwrap();
        assert_eq!(c.r, 1.);
        assert_eq!(c.g, 1.);
        assert_eq!(c.b, 1.);
        assert_eq!(c.a, 1.);
    }
}
