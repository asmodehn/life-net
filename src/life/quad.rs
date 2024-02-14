use crate::life::cell;
use crate::life::cell::{ALIVE, DEAD};
use crate::perf::DurationAverage;
use itertools::iproduct;
use macroquad::prelude::Image;
use macroquad::rand::ChooseRandom;
use std::time::Duration;

pub struct Quad {
    pub image: Image,
    average_duration: DurationAverage,
    //TODO : try grid crate here ??
    original: Vec<[u8; 4]>,
    left_over: Vec<(u16, u16)>,
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
            average_duration: DurationAverage::default(),
            original: vec![],
            left_over: vec![],
        }
    }

    pub(crate) fn update(
        &mut self,
        _elapsed: Duration,
        available: Duration,
        until: impl Fn(&Self) -> bool,
    ) -> &mut Self {
        self.average_duration.timed_start();
        //
        // if self.average_duration.avg().unwrap_or_default() < available {
        //     if self.update == UpdateKind::partial {
        //         // println!("{:?} => ATTEMPT COMPLETE UPDATE IN ONE CALL", available);
        //         self.update = UpdateKind::total;
        //     }
        //     self.next
        //         .compute(&mut self.image, |partial_quad: &PartialQuad| {
        //             partial_quad.is_ready()
        //         });
        // } else {
        //     if self.update == UpdateKind::total {
        //         // println!("{:?} => NOT ENOUGH TIME FOR FULL UPDATE", available);
        //         self.update = UpdateKind::partial;
        //     }
        //     self.next
        //         .compute(&mut self.image, |_partial_quad: &PartialQuad| {
        //             self.average_duration.timed_elapsed() > available
        //         });
        // }
        //

        let w = self.image.width;
        let h = self.image.height;

        loop {
            let mut nxt = self.left_over.pop();
            if nxt.is_none() {
                self.left_over = iproduct!(0..h, 0..w).collect();
                self.left_over.shuffle();

                self.original = self.image.get_image_data().into();

                // continue immediately with another update without waiting.
                // if a break is desired, `until` should be used.
                nxt = self.left_over.pop();
            }

            let (y, x) = nxt.unwrap();
            let updated_color = cell::update_on_quad(&*self.original, x as i32, y as i32, w, h);
            //CAREFUL : grid computation here must be exactly same as image...

            self.image.set_pixel(x as u32, y as u32, updated_color);

            //late exit to have initialization only inside the loop.
            if until(self) {
                break;
            }
        }

        self.average_duration.timed_stop();
        self
    }

    //TODO  :Stepper trait !
    // pub fn prepare(&mut self) {
    //
    // }
    //
    // pub fn step(&mut self) {
    //
    // }
    //
    pub fn completed(&self) -> bool {
        self.left_over.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::life::cell;
    use crate::life::cell::{ALIVE, DEAD};
    use crate::life::quad::Quad;
    use std::time::Duration;

    use test::Bencher;

    #[test]
    fn cell_dies_alone() {
        let mut q = Quad::new(1, 1);

        q.image.update(&[cell::ALIVE]);

        //one update
        q.update(Duration::new(0, 0), Duration::MAX, |qq| qq.completed());

        assert_eq!(q.image.get_pixel(0, 0), cell::DEAD)
    }

    #[test]
    fn cell_dies_alone_minimal_update() {
        let mut q = Quad::new(1, 1);

        q.image.update(&[cell::ALIVE]);

        q.update(Duration::new(0, 0), Duration::MAX, |_s| true);

        assert_eq!(q.image.get_pixel(0, 0), DEAD);
    }

    #[test]
    fn check_stationary_squad() {
        let mut q = Quad::new(2, 2);
        //permanent square in quad
        q.image.update(&[cell::ALIVE; 4]);

        //one update
        q.update(Duration::new(0, 0), Duration::MAX, |qq| qq.completed());

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
        q.update(Duration::new(0, 0), Duration::MAX, |_s| true);

        assert_eq!(q.image.get_pixel(0, 0), cell::ALIVE);
        assert_eq!(q.image.get_pixel(0, 1), cell::ALIVE);
        assert_eq!(q.image.get_pixel(1, 0), cell::ALIVE);
        assert_eq!(q.image.get_pixel(1, 1), cell::ALIVE);
    }

    #[bench]
    fn bench_update_064_064(b: &mut Bencher) {
        let mut q = Quad::new(64, 64);

        b.iter(|| {
            q.update(Duration::new(0, 0), Duration::MAX, move |qq| qq.completed());
        });
    }

    #[bench]
    fn bench_update_128_128(b: &mut Bencher) {
        let mut q = Quad::new(128, 128);

        b.iter(|| {
            q.update(Duration::new(0, 0), Duration::MAX, move |qq| qq.completed());
        });
    }

    #[bench]
    fn bench_update_256_256(b: &mut Bencher) {
        let mut q = Quad::new(256, 256);

        b.iter(|| {
            q.update(Duration::new(0, 0), Duration::MAX, move |qq| qq.completed());
        });
    }
}
