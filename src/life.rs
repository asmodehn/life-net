use macroquad::color::{BLACK, WHITE};
use macroquad::prelude::Image;

// use crate::engine::Updatable;
use crate::render::Renderable;
use std::time::Duration;

use std::convert::TryFrom;
use std::{u16, usize};

mod cell;

fn usize_from_i32(v: i32) -> usize {
    usize::try_from(v).unwrap()
}

fn u16_from_usize(v: usize) -> u16 {
    u16::try_from(v).unwrap()
}

pub struct World {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<cell::State>,
    pub buffer: Vec<cell::State>,
    pub image: Image,
}

impl World {
    //size of your world depend on your platform
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

    pub(crate) fn update(&mut self, _elapsed: Duration) {
        let w = self.width;
        let h = self.height;

        for y in 0..h {
            for x in 0..w {
                self.buffer[y * w + x] = cell::update(&self.cells, x as i32, y as i32, w, h);
            }
        }
        for i in 0..self.buffer.len() {
            // TODO : move tis somewhere else ?
            self.cells[i] = self.buffer[i];
        }
    }
}

impl Renderable for World {
    //TODO : review lifetime once structure is decided
    fn render<'s>(&'s self, buffer: &'s mut Image) -> &'s Image {
        for i in 0..self.buffer.len() {
            buffer.set_pixel(
                (u16_from_usize(i) % buffer.width) as u32,
                (u16_from_usize(i) / buffer.width) as u32,
                match self.buffer[i as usize] {
                    cell::State::Alive => BLACK,
                    cell::State::Dead => WHITE,
                },
            );
        }
        buffer
    }
}

#[allow(dead_code)]
pub fn run(w: &mut World) {
    loop {
        //TODO : actual duration...
        w.update(Duration::new(0, 0));
    }
}
