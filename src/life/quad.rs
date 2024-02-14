mod partial;

use crate::life::cell;
use crate::life::cell::{ALIVE, DEAD};
use crate::perf::DurationAverage;
use macroquad::prelude::Image;
use std::time::Duration;

use crate::life::quad::partial::PartialQuad;

#[derive(PartialEq)]
enum UpdateKind {
    total,
    partial,
}

pub struct Quad {
    pub image: Image,
    next: PartialQuad,
    average_duration: DurationAverage,
    update: UpdateKind,
}

impl Quad {
    pub fn new(width: u16, height: u16) -> Self {
        let mut new_quad = Image::gen_image_color(width, height, DEAD);
        let last = new_quad.clone();

        let next_quad = PartialQuad::new(width, height, ALIVE); // TMP DEBUG: should be DEAD like image...

        //TODO : data initializer as parameter...
        for pix in new_quad.get_image_data_mut().iter_mut() {
            if macroquad::prelude::rand::gen_range(0, 5) == 0 {
                *pix = cell::color(cell::State::Alive).into();
            }
        }

        Self {
            image: new_quad,
            next: next_quad,
            average_duration: DurationAverage::default(),
            update: UpdateKind::total,
        }
    }

    pub(crate) fn update(&mut self, _elapsed: Duration, available: Duration) {
        self.average_duration.timed_start();

        if self.average_duration.avg().unwrap_or_default() < available {
            if self.update == UpdateKind::partial {
                // println!("{:?} => ATTEMPT COMPLETE UPDATE IN ONE CALL", available);
                self.update = UpdateKind::total;
            }
            self.next
                .compute(&mut self.image, |partial_quad: &PartialQuad| {
                    partial_quad.is_ready()
                });
        } else {
            if self.update == UpdateKind::total {
                // println!("{:?} => NOT ENOUGH TIME FOR FULL UPDATE", available);
                self.update = UpdateKind::partial;
            }
            self.next
                .compute(&mut self.image, |_partial_quad: &PartialQuad| {
                    self.average_duration.timed_elapsed() > available
                });
        }

        self.average_duration.timed_stop();
    }
}

#[cfg(test)]
mod tests {
    use crate::life::cell;
    use crate::life::cell::State;
    use crate::life::quad::Quad;
    use std::time::Duration;

    use test::Bencher;

    #[test]
    fn lonely_dying_quad() {
        let mut q = Quad::new(1, 1);

        q.image.update(&[cell::ALIVE]);

        //one update
        q.update(Duration::new(0, 0), Duration::MAX);

        assert_eq!(q.image.get_pixel(0, 0), cell::DEAD)
    }
    #[test]
    fn check_stationary_one() {
        let mut q = Quad::new(2, 2);
        //permanent square in quad
        q.image.update(&[cell::ALIVE; 4]);

        //one update
        q.update(Duration::new(0, 0), Duration::MAX);

        assert_eq!(q.image.get_pixel(0, 0), cell::color(State::Alive));
        assert_eq!(q.image.get_pixel(0, 1), cell::color(State::Alive));
        assert_eq!(q.image.get_pixel(1, 0), cell::color(State::Alive));
        assert_eq!(q.image.get_pixel(1, 1), cell::color(State::Alive));
    }

    #[bench]
    fn bench_update_064_064(b: &mut Bencher) {
        let mut q = Quad::new(64, 64);

        b.iter(|| q.update(Duration::new(0, 0), Duration::MAX));
    }

    #[bench]
    fn bench_update_128_128(b: &mut Bencher) {
        let mut q = Quad::new(128, 128);

        b.iter(|| q.update(Duration::new(0, 0), Duration::MAX));
    }

    #[bench]
    fn bench_update_256_256(b: &mut Bencher) {
        let mut q = Quad::new(256, 256);

        b.iter(|| q.update(Duration::new(0, 0), Duration::MAX));
    }
}
