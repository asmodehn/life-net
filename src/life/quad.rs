use crate::life::cell;
use crate::life::cell::{ALIVE, DEAD};
use itertools::iproduct;
use macroquad::color::Color;
use macroquad::prelude::Image;
use macroquad::rand::ChooseRandom;
use std::ops::Deref;
use std::time::{Duration, Instant};

struct ImageUpdate {
    width: u16,
    height: u16,
    //TODO : try grid crate here ??
    original: Vec<[u8; 4]>,
    left_over: Vec<(u16, u16)>,
}

impl ImageUpdate {
    pub fn new(image: &Image) -> Self {
        let height = image.height;
        let width = image.width;

        let original = image.get_image_data().into();

        let mut left_over: Vec<(u16, u16)> = iproduct!(0..height, 0..width).collect();
        left_over.shuffle();

        Self {
            width,
            height,
            original,
            left_over,
        }
    }

    //TODO : pass iterator as parameter here...
    fn step(&mut self) -> Option<(u16, u16, Color)> {
        match self.left_over.pop() {
            None => None,
            Some((y, x)) => Some((
                x,
                y,
                cell::update_on_quad(
                    self.original.as_slice(),
                    x as i32,
                    y as i32,
                    self.width,
                    self.height,
                ),
            )), //CAREFUL : grid computation here must be exactly same as image...
        }
    }

    fn completed(&self) -> bool {
        self.left_over.is_empty()
    }
}

pub struct Quad {
    pub image: Image,
    update: Option<ImageUpdate>,
    updated_cb: fn(&mut Self),
}

impl Quad {
    pub fn new(width: u16, height: u16) -> Self {
        let mut new_quad = Image::gen_image_color(width, height, DEAD);

        //TODO : data initializer as parameter...
        for pix in new_quad.get_image_data_mut().iter_mut() {
            if macroquad::prelude::rand::gen_range(0, 5) == 0 {
                *pix = cell::color(cell::State::Alive).into();
            }
        }

        Self {
            image: new_quad,
            update: None,
            updated_cb: Self::reset_update,
        }
    }

    fn with_updated_cb(self, cb: fn(&mut Self)) -> Self {
        Self {
            updated_cb: cb,
            ..self
        }
    }

    fn reset_update(&mut self) {
        self.update = Some(ImageUpdate::new(&self.image));
    }

    pub fn is_updated(&self) -> bool {
        self.update.as_ref().is_some_and(|iup| iup.completed())
    }

    // Maybe : cleaner API instead of Option ??
    pub(crate) fn compute_once(&mut self) {
        // TODO : pass iterator (explicit loop) THEN rename -> update !
        loop {
            //check for completion
            if self.is_updated() {
                (self.updated_cb)(self);
                break;
            }
            //TODO : test: isn't this duplication ??
            //(re)initialize if needed
            if self.update.is_none() {
                //start new update when necessary
                self.update = Some(ImageUpdate::new(&self.image));
            }

            //attempt an update step
            match self.update.as_mut().unwrap().step() {
                None => {}
                Some((x, y, cell_color)) => {
                    self.image.set_pixel(x as u32, y as u32, cell_color);
                }
            }
        }
    }

    // TODO : rename -> update_until !
    pub(crate) fn compute_once_or_until(&mut self, mut until: impl FnMut(&Self) -> bool) {
        loop {
            //check for completion
            if self.is_updated() {
                (self.updated_cb)(self);
                break;
            }
            //TODO : test: isn't this duplication ??
            //(re)initialize if needed
            if self.update.is_none() {
                //start new update when necessary
                self.update = Some(ImageUpdate::new(&self.image));
            }

            //attempt an update step
            match self.update.as_mut().unwrap().step() {
                None => {}
                Some((x, y, cell_color)) => {
                    self.image.set_pixel(x as u32, y as u32, cell_color);
                }
            }

            //late until to ensure some progress
            //can mutate itself !
            if until(self) {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::life::cell;
    use crate::life::cell::{ALIVE, DEAD};
    use crate::life::quad::Quad;
    use std::time::Duration;

    use test::Bencher;
    use test::RunIgnored::No;

    #[test]
    fn cell_dies_alone() {
        let mut q = Quad::new(1, 1);

        q.image.update(&[cell::ALIVE]);

        //one update
        q.compute_once();

        assert_eq!(q.image.get_pixel(0, 0), cell::DEAD)
    }

    #[test]
    fn cell_dies_alone_minimal_update() {
        let mut q = Quad::new(1, 1);

        q.image.update(&[cell::ALIVE]);

        q.compute_once_or_until(|_s| true);

        assert_eq!(q.image.get_pixel(0, 0), DEAD);
    }

    #[test]
    fn check_stationary_squad() {
        let mut q = Quad::new(2, 2);
        //permanent square in quad
        q.image.update(&[cell::ALIVE; 4]);

        //one update
        q.compute_once();

        assert_eq!(q.image.get_pixel(0, 0), cell::ALIVE);
        assert_eq!(q.image.get_pixel(0, 1), cell::ALIVE);
        assert_eq!(q.image.get_pixel(1, 0), cell::ALIVE);
        assert_eq!(q.image.get_pixel(1, 1), cell::ALIVE);
    }

    #[test]
    fn check_stationary_squad_minimal_update() {
        let mut q = Quad::new(2, 2);
        //permanent square in quad
        q.image.update(&[cell::ALIVE; 4]);

        //one update
        q.compute_once_or_until(|_s| true);

        assert_eq!(q.image.get_pixel(0, 0), cell::ALIVE);
        assert_eq!(q.image.get_pixel(0, 1), cell::ALIVE);
        assert_eq!(q.image.get_pixel(1, 0), cell::ALIVE);
        assert_eq!(q.image.get_pixel(1, 1), cell::ALIVE);
    }

    #[bench]
    fn bench_update_064_064(b: &mut Bencher) {
        let mut q = Quad::new(64, 64);

        b.iter(|| {
            q.compute_once();
        });
    }

    #[bench]
    fn bench_update_128_128(b: &mut Bencher) {
        let mut q = Quad::new(128, 128);

        b.iter(|| {
            q.compute_once();
        });
    }

    #[bench]
    fn bench_update_256_256(b: &mut Bencher) {
        let mut q = Quad::new(256, 256);

        b.iter(|| {
            q.compute_once();
        });
    }
}
