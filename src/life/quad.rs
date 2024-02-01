use crate::life::cell;
use crate::render::Renderable;
use macroquad::color::RED;
use macroquad::prelude::{Color, Image};
use std::mem::swap;
use std::time::Duration;

pub struct ColorStateBuffer {
    //TODO
}

pub struct Quad {
    pub image: Image,
    next: Image, //TODO : ColorStateBuffer might be simpler to work with && safer
}

impl Quad {
    pub fn new(width: u16, height: u16) -> Quad {
        let dead_color = Color::from_hex(cell::to_hex(cell::color(cell::State::Dead)));
        println!("DEAD: {:#?}", dead_color);
        println!(
            "ALIVE: {:#?}",
            Color::from_hex(cell::to_hex(cell::color(cell::State::Alive)))
        );

        let mut new_quad = Image::gen_image_color(width, height, dead_color);

        //TODO : data initializer as parameter...
        for pix in new_quad.get_image_data_mut().iter_mut() {
            if macroquad::prelude::rand::gen_range(0, 5) == 0 {
                *pix = cell::color(cell::State::Alive);
            }
        }

        Quad {
            image: new_quad,
            next: Image::empty(),
        }
    }

    pub(crate) fn update(&mut self, _elapsed: Duration) {
        let w = self.image.width;
        let h = self.image.height;

        self.next.clone_from(&self.image);

        let buffer = self.next.get_image_data_mut();

        for y in 0..h {
            for x in 0..w {
                //TODO : check overflow here...
                buffer[(y as usize * w as usize + x as usize)] =
                    cell::update_on_quad(buffer, x as i32, y as i32, w, h);
            }
        }

        self.swapbuf();
    }

    //TODO : safe pixel accessor...

    pub fn swapbuf(&mut self) {
        //TODO : if image were sized, we could std::mem::replace
        // self.image.get_image_data_mut().copy_from_slice(self.next.get_image_data());

        for i in 0..self.next.get_image_data().len() {
            // TODO : move this somewhere else ?
            self.image.get_image_data_mut()[i] = self.next.get_image_data()[i];
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::life::cell;
    use crate::life::cell::State;
    use crate::life::quad::Quad;
    use macroquad::prelude::Color;

    fn mini_quad() {
        let q = Quad::new(1, 1);

        assert_eq!(
            q.image.get_pixel(1, 1),
            Color::from_hex(cell::to_hex(cell::color(State::Dead)))
        )
    }
    #[test]
    fn check_stationary_one() {}

    fn check_oscillating_one() {}
}
