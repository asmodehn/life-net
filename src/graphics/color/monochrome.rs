use once_cell::sync::Lazy;
use std::collections::Bound;
use std::fmt::Debug;
use std::ops::Bound::Included;
use std::ops::{RangeBounds, RangeInclusive};

/// A type alias, because these are exactly the same (including possible value range).
pub type ColorByte = u8;

/// A zero-cost Newtype to implement a chromatic scale on a f32 from 0.0 to 1.0
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Monochrome(f32);

impl Default for Monochrome {
    fn default() -> Self {
        Self(*Self::RANGE.start())
    }
}

impl Monochrome {
    const RANGE: RangeInclusive<f32> = 0f32..=1f32;

    #[inline]
    fn into_inner(self) -> f32 {
        self.0
    }
}

//For relaxed Equality checks
impl PartialEq<f32> for Monochrome {
    fn eq(&self, other: &f32) -> bool {
        self.0 == *other
    }
}

//IMPL From reflexively
impl From<Monochrome> for f32 {
    fn from(value: Monochrome) -> Self {
        value.into_inner()
    }
}

//From the other way can fail -> impl of  try_from
impl TryFrom<f32> for Monochrome {
    type Error = String; // TODO : proper Error type (like TryFromIntError)

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if Monochrome::RANGE.contains(&value) {
            Ok(Monochrome(value))
        } else {
            Err(format!("{:?} is not contained in {:?}", value, Monochrome::RANGE).to_owned())
        }
    }
}

const SCALE_COEF_BYTE_TO_F32: Lazy<f32> = Lazy::new(|| {
    (Monochrome::RANGE.end() - Monochrome::RANGE.start()) / (ColorByte::MAX - ColorByte::MIN) as f32
});

impl From<ColorByte> for Monochrome {
    fn from(value: ColorByte) -> Self {
        Monochrome(
            (value - ColorByte::MIN) as f32 * SCALE_COEF_BYTE_TO_F32.to_owned()
                + Monochrome::RANGE.start(),
        )
    }
}

const SCALE_COEF_F32_TO_BYTE: Lazy<f32> = Lazy::new(|| {
    (ColorByte::MAX - ColorByte::MIN) as f32 / (Monochrome::RANGE.end() - Monochrome::RANGE.start())
});

impl From<Monochrome> for ColorByte {
    fn from(value: Monochrome) -> Self {
        ((value.into_inner() - Monochrome::RANGE.start()) * SCALE_COEF_F32_TO_BYTE.to_owned())
            as ColorByte
            + ColorByte::MIN
    }
}

#[cfg(test)]
mod tests {
    use crate::graphics::color::monochrome::{ColorByte, Monochrome};
    use test::Bencher;

    #[test]
    fn check_monochrome_from_u8() {
        let g = Monochrome::from(255u8);
        assert_eq!(g, 1f32)
    }

    #[test]
    fn check_u8_into_monochrome() {
        let g: Monochrome = 255u8.into();
        assert_eq!(g.into_inner(), 1f32)
    }

    #[test]
    fn check_monochrome_from_f32() {
        let g = Monochrome::try_from(1f32).unwrap();
        assert_eq!(g.into_inner(), 1f32)
    }
    #[test]
    fn check_f32_into_monochrome() {
        let g: Monochrome = 1f32.try_into().unwrap();
        assert_eq!(g.into_inner(), 1f32)
    }

    #[test]
    fn check_colorbyte_from_monochrome() {
        let m = Monochrome::try_from(1f32).unwrap();
        let g = ColorByte::from(m);
        assert_eq!(g, 255u8)
    }
    #[test]
    fn check_monochrome_into_colorbyte() {
        let m = Monochrome::try_from(1f32).unwrap();
        let g: ColorByte = m.into();
        assert_eq!(g, 255u8)
    }
}
