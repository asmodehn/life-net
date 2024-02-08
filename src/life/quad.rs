use crate::life::cell;
use crate::life::cell::{ALIVE, DEAD};
use crate::life::world::usize_from_u16;
use crate::render::Renderable;
use macroquad::color::Color;
use macroquad::prelude::Image;
use std::time::Duration;

pub struct Quad {
    pub image: Image,
    next: Vec<Color>, // dynamic or/and on the heap, just to bypass parameterizing with size...
                      //TODO : try grid crate here ??
}

impl Quad {
    pub fn new(width: u16, height: u16) -> Quad {
        //TMP debug
        let dead_color = cell::color(cell::State::Dead);
        println!("DEAD: {:#?}", dead_color);
        println!("ALIVE: {:#?}", cell::color(cell::State::Alive));

        let mut new_quad = Image::gen_image_color(width, height, dead_color);

        let next_vec = vec![DEAD; usize_from_u16(width) * usize_from_u16(height)];

        //TODO : data initializer as parameter...
        for pix in new_quad.get_image_data_mut().iter_mut() {
            if macroquad::prelude::rand::gen_range(0, 5) == 0 {
                *pix = cell::color(cell::State::Alive).into();
            }
        }

        Quad {
            image: new_quad,
            next: next_vec,
        }
    }

    pub(crate) fn update(&mut self, _elapsed: Duration) {
        let w = self.image.width;
        let h = self.image.height;

        for y in 0..h {
            for x in 0..w {
                //TODO : check overflow here...
                self.next[usize_from_u16(y) * usize_from_u16(w) + usize_from_u16(x)] =
                    cell::update_on_quad(self.image.get_image_data(), x as i32, y as i32, w, h);
            }
        }

        self.image.update(&self.next.as_slice());
    }
}

#[cfg(test)]
mod tests {
    use crate::life::cell;
    use crate::life::cell::State;
    use crate::life::quad::Quad;
    use std::time::Duration;

    #[test]
    fn lonely_dying_quad() {
        let mut q = Quad::new(1, 1);

        q.image.update(&[cell::ALIVE]);

        //one update
        q.update(Duration::new(0, 0));

        assert_eq!(q.image.get_pixel(0, 0), cell::DEAD)
    }
    #[test]
    fn check_stationary_one() {
        let mut q = Quad::new(2, 2);
        //permanent square in quad
        q.image.update(&[cell::ALIVE; 4]);

        //one update
        q.update(Duration::new(0, 0));

        assert_eq!(q.image.get_pixel(0, 0), cell::color(State::Alive));
        assert_eq!(q.image.get_pixel(0, 1), cell::color(State::Alive));
        assert_eq!(q.image.get_pixel(1, 0), cell::color(State::Alive));
        assert_eq!(q.image.get_pixel(1, 1), cell::color(State::Alive));
    }
}
