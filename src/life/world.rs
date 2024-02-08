use crate::life::cell;
use crate::render::Renderable;
use macroquad::color::{BLACK, WHITE};
use macroquad::prelude::Image;
use std::time::Duration;

pub(crate) fn usize_from_i32(v: i32) -> usize {
    usize::try_from(v).unwrap()
}

pub(crate) fn u16_from_usize(v: usize) -> u16 {
    u16::try_from(v).unwrap()
}
pub(crate) fn usize_from_u16(v: u16) -> usize {
    usize::try_from(v).unwrap()
}

//TODO : a grid of images (u16 * u16), modifiable as a single image ( ...

pub struct World {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<cell::State>,
    pub buffer: Vec<cell::State>,
    pub image: Image, // TODO : scene graph to hold multiple images with relative positions...
}

impl World {
    //size of your world depend on your platform

    #[allow(dead_code)]
    pub fn new(world_width: usize, world_height: usize) -> World {
        let world_size = match world_width.overflowing_mul(world_height) {
            (res, false) => res,
            (_, true) => {
                println!("WARNING: World size too big !");
                usize::MAX
            }
        };

        let mut world = World {
            width: world_width,
            height: world_height,
            cells: vec![cell::State::Dead; world_size],
            buffer: vec![cell::State::Dead; world_size],
            //TODO : multiple images if usize > u16
            image: Image::gen_image_color(world_width as u16, world_height as u16, WHITE),
        };

        for cell in world.cells.iter_mut() {
            if macroquad::prelude::rand::gen_range(0, 5) == 0 {
                *cell = cell::State::Alive;
            }
        }
        return world;
    }

    #[allow(dead_code)]
    pub(crate) fn update(&mut self, _elapsed: Duration) {
        let w = self.width;
        let h = self.height;

        for y in 0..h {
            for x in 0..w {
                self.buffer[y * w + x] = cell::update(&self.cells, x as i32, y as i32, w, h);
            }
        }
        for i in 0..self.buffer.len() {
            // TODO : move this somewhere else ?
            self.cells[i] = self.buffer[i];
        }
    }
}

impl Renderable for World {
    //TODO : review lifetime once structure is decided
    fn render(&mut self) -> &Image {
        for i in 0..self.buffer.len() {
            self.image.set_pixel(
                (u16_from_usize(i) % self.image.width) as u32,
                (u16_from_usize(i) / self.image.width) as u32,
                match self.buffer[i] {
                    cell::State::Alive => BLACK,
                    cell::State::Dead => WHITE,
                },
            );
        }
        &self.image
    }
}
