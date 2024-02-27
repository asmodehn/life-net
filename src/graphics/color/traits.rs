use std::ops::{Range, RangeBounds};

//TODO : Colllorscale -> grayscale ??
//
// pub trait FromColor<T: Colorscale> { // TODO: rename -> ColorFrom less confusing ?
// fn from_color_value(value: T) -> Self;
// }
//
// pub trait IntoColor<T: Colorscale> { // TODO : rename -> ColorInto less confusing ?
// fn into_color_value(self: Self) -> T;
// }
//
//
// //IMPLS FromColor and IntoColor
// impl FromColor<u8> for u8 {
//     #[inline]
//     fn from_color_value(value: u8) -> Self {
//         value
//     }
// }
//
// impl IntoColor<u8> for u8 {
//     #[inline]
//     fn into_color_value(self: Self) -> u8 {
//         self
//     }
// }
//
// impl FromColor<f32> for f32 {
//     #[inline]
//     fn from_color_value(value: f32) -> Self {
//         value
//     }
// }
//
// impl IntoColor<f32> for f32 {
//     #[inline]
//     fn into_color_value(self: Self) -> f32 {
//         self
//     }
// }
//
// //actual conversions
// impl FromColor<u8> for f32 {
//     #[inline]
//     fn from_color_value(value: u8) -> Self {
//         value as f32 / 255.
//     }
// }
//
// impl IntoColor<f32> for u8 {
//     #[inline]
//     fn into_color_value(self: Self) -> f32 {
//         self as f32 / 255.
//     }
// }
//
// impl FromColor<f32> for u8 {
//     #[inline]
//     fn from_color_value(value: f32) -> Self {
//         (value * 255.) as u8
//     }
// }
//
// impl IntoColor<u8> for f32 {
//     #[inline]
//     fn into_color_value(self: Self) -> u8 {
//         (self * 255.) as u8
//     }
// }
