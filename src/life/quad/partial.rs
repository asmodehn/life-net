use crate::life::cell;
use crate::life::cell::{ALIVE, DEAD};
use crate::life::world::usize_from_u16;
use itertools::iproduct;
use macroquad::color::Color;
use macroquad::prelude::Image;
use macroquad::rand::ChooseRandom;
use rand::rngs::ThreadRng;
use rand::thread_rng;
use std::ops::Deref;

///
/// A Quad that is partially updateable...
/// To make it nicer to look at the order of grid cell update is randomized.
/// If you cn do the update in one update call, just use a simple quad (faster)
///
#[derive(Default)]
pub struct PartialQuad {
    width: u16,
    height: u16,
    holder: Vec<Color>, // dynamic or/and on the heap, just to bypass parameterizing with size...
    //TODO : try grid crate here ??
    original: Vec<[u8; 4]>,
    left_over: Vec<(u16, u16)>,
}

impl PartialQuad {
    #[allow(dead_code)]
    fn default() -> Self {
        PartialQuad {
            width: 0,
            height: 0,
            holder: vec![],
            original: vec![],
            left_over: vec![],
        }
    }

    pub fn new(width: u16, height: u16, color: Color) -> Self {
        let next_vec = vec![color; usize_from_u16(width) * usize_from_u16(height)];

        PartialQuad {
            width,
            height,
            holder: next_vec,
            original: vec![],
            left_over: vec![],
        }
    }

    pub fn is_ready(&self) -> bool {
        self.left_over.is_empty()
    }

    pub fn compute(&mut self, image: &mut Image, until: impl Fn(&Self) -> bool) -> &mut Self {
        let w = self.width;
        let h = self.height;

        loop {
            let mut nxt = self.left_over.pop();
            if nxt.is_none() {
                self.left_over = iproduct!(0..self.height, 0..self.width).collect();
                self.left_over.shuffle();

                self.original = image.get_image_data().into();

                // continue immediately with another update without waiting.
                // if a break is desired, `until` should be used.
                nxt = self.left_over.pop();
            }

            let (y, x) = nxt.unwrap();
            let updated_color = cell::update_on_quad(&*self.original, x as i32, y as i32, w, h);
            //CAREFUL : grid computation here must be exactly same as image...

            image.set_pixel(x as u32, y as u32, updated_color);

            //late exit to have initialization only inside the loop.
            if until(self) {
                break;
            }
        }

        //TODO : BUG FIX : NOT THE WHOLE IMAGE !
        //update image immediately after one compute pass
        // image.update(&self.holder.as_slice());
        //
        self
    }

    // pub fn get_color(&self, x: u16, y: u16) -> Color {
    //     self.holder[usize_from_u16(y) * usize_from_u16(self.width) + usize_from_u16(x)]
    //
    //
    // }
}

//TODO : for simple indexing inside holder...
// impl<Idx> Index<Idx> for PartialQuad
//     where
//         // TODO : for clean usage of ranges, etc.
//         // Idx: (SliceIndex<[Color], Output = Color>),
//         Idx: (u16, u16),
// {
//     type Output = Color;
//
//     #[inline(always)]
//     fn index(&self, index: Idx) -> &Self::Output {
//         self.holder.index(index)
//     }
// }

#[cfg(test)]
mod tests {
    use crate::life::cell::{ALIVE, DEAD};

    use super::*;

    #[test]
    fn cell_dies_alone() {
        let mut pq = PartialQuad::new(1, 1, ALIVE);

        let mut img = Image::gen_image_color(1, 1, ALIVE);

        pq.compute(&mut img, |s| s.is_ready());

        assert_eq!(img.get_pixel(0, 0), DEAD);
    }

    #[test]
    fn cell_dies_alone_update_strictly_monotonic() {
        let mut pq = PartialQuad::new(1, 1, ALIVE);

        let mut img = Image::gen_image_color(1, 1, ALIVE);

        pq.compute(&mut img, |s| true);

        assert_eq!(img.get_pixel(0, 0), DEAD);
    }

    #[test]
    fn squad_survives() {
        let mut pq = PartialQuad::new(2, 2, ALIVE);

        let mut img = Image::gen_image_color(2, 2, ALIVE);

        pq.compute(&mut img, |s| s.is_ready());

        assert_eq!(img.get_pixel(0, 0), ALIVE);
        assert_eq!(img.get_pixel(1, 0), ALIVE);
        assert_eq!(img.get_pixel(0, 1), ALIVE);
        assert_eq!(img.get_pixel(1, 1), ALIVE);
    }

    #[test]
    fn minimal_update_doesnt_annihilate() {
        let mut pq = PartialQuad::new(2, 2, ALIVE);

        let mut img = Image::gen_image_color(2, 2, ALIVE);

        pq.compute(&mut img, |s| true);

        assert_eq!(img.get_pixel(0, 0), ALIVE);
        assert_eq!(img.get_pixel(1, 0), ALIVE);
        assert_eq!(img.get_pixel(0, 1), ALIVE);
        assert_eq!(img.get_pixel(1, 1), ALIVE);
    }
}
