use crate::life::cell;
use crate::life::world::usize_from_u16;
use crate::render::Renderable;
use macroquad::prelude::Image;
use std::time::Duration;

pub struct Quad {
    pub image: Image,
    next: Image, //TODO : [Color] might be simpler to work with (see Image::update())
    next_color: Vec<macroquad::color::Color>,
}

impl Quad {
    pub fn new(width: u16, height: u16) -> Quad {
        //TMP debug
        let dead_color = cell::color(cell::State::Dead);
        println!("DEAD: {:#?}", dead_color);
        println!("ALIVE: {:#?}", cell::color(cell::State::Alive));

        let mut new_quad = Image::gen_image_color(width, height, dead_color);

        //TODO : data initializer as parameter...
        for pix in new_quad.get_image_data_mut().iter_mut() {
            if macroquad::prelude::rand::gen_range(0, 5) == 0 {
                *pix = cell::color(cell::State::Alive).into();
            }
        }

        Quad {
            image: new_quad,
            next: Image::empty(),
            next_color: vec![], // TODO : proper size !
        }
    }

    pub(crate) fn update(&mut self, _elapsed: Duration) {
        let w = self.image.width;
        let h = self.image.height;

        self.next.clone_from(&self.image);

        //TODO : to simplify
        // for y in 0..h {
        //     for x in 0..w {
        //         //TODO : check overflow here...
        //         self.next_color[usize_from_u16(y) * usize_from_u16(w) + usize_from_u16(x)] = cell::update_on_quad(self.image.get_image_data(), x as i32, y as i32, w, h);
        //     }
        // }

        let buffer = self.next.get_image_data_mut();

        for y in 0..h {
            for x in 0..w {
                //TODO : check overflow here...
                buffer[usize_from_u16(y) * usize_from_u16(w) + usize_from_u16(x)] =
                    cell::update_on_quad(self.image.get_image_data(), x as i32, y as i32, w, h)
                        .into();
            }
        }

        self.swapbuf();
    }

    //TODO : safe pixel accessor... NO NEED ? use Image::update([Color]) instead...

    pub fn swapbuf(&mut self) {
        //TODO : if image were sized, we could std::mem::replace
        // self.image.get_image_data_mut().copy_from_slice(self.next.get_image_data());

        // other option: basic cloning
        self.image.clone_from(&self.next);

        // for i in 0..self.next.get_image_data().len() {
        //     // TODO : move this somewhere else ?
        //     self.image.get_image_data_mut()[i] = self.next.get_image_data()[i];
        // }
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
