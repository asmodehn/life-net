use crate::graphics::quad::{Drawable, Placed, Quad};
use macroquad::color::{Color, RED, YELLOW};
use macroquad::math::{IVec2, UVec2};
use macroquad::prelude::{
    clear_background, draw_texture, get_fps, get_frame_time, next_frame, Image, Texture2D, BLUE,
};
use macroquad::ui;
use std::cell::{Cell, Ref, RefCell};
use std::default::Default;
use std::ops::{Deref, MulAssign};
use std::time::Duration;

//DO we really need ot depend on Scene here ? how about using placedquad and drawable  trait here ?
use crate::graphics::scene;
use crate::graphics::scene::PlacedSprite;
use crate::graphics::scene::Scene;

pub trait Viewable {
    // fn render(&mut self) -> Box<dyn Drawable>;
    fn rendered(&self) -> &Image;
}

const DEFAULT_COLOR: Color = BLUE;
const DEFAULT_FRAMERATE: i32 = 60;
const DEFAULT_DIMENSIONS: UVec2 = UVec2::new(128, 128); // TODO : appropriate default ?

#[derive(Clone, Copy)]

pub struct View {
    // ui: Ui, // TODO
    background: Color,
    dimensions: UVec2,
    pub(crate) target_fps: i32,
}

impl Default for View {
    fn default() -> Self {
        Self {
            background: DEFAULT_COLOR,
            dimensions: DEFAULT_DIMENSIONS,
            target_fps: DEFAULT_FRAMERATE,
        }
    }
}

impl View {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn with_dimensions(self, width: u32, height: u32) -> Self {
        Self {
            dimensions: UVec2::new(width, height),
            ..self
        }
    }

    pub(crate) fn with_framerate(self, target_fps: i32) -> Self {
        Self { target_fps, ..self }
    }

    //TODO : "render" (the term) should include shaders... is View appropriate for this term ??
    pub(crate) async fn render(
        &self,
        placed_quads: impl Iterator<Item = Ref<'_, scene::PlacedDrawable>>,
    ) {
        for pq in placed_quads {
            pq.draw(pq.get_position());
        }
        // placed_quads.map(|pq| pq.draw(pq.get_position));

        //TODO : on screen / window instead of log...
        // println!("FPS: {}", self.current_fps());

        //CAREFUL with z order !
        ui::root_ui().label(None, format!("FPS: {}", self.current_fps()).as_str());

        next_frame().await;
    }

    pub fn last_frame_time(&self) -> Duration {
        Duration::from_secs_f32(get_frame_time())
    }

    #[allow(dead_code)]
    pub fn current_fps(&self) -> i32 {
        get_fps()
        //TODO : average these over time... in update ? only when used ?
    }

    pub(crate) fn target_frame_time(&self) -> Duration {
        Duration::from_secs_f32(1. / self.target_fps as f32)
    }
}

impl Quad for View {
    fn get_dimensions(&self) -> UVec2 {
        self.dimensions
    }

    fn get_background(&self) -> Color {
        self.background
    }

    fn set_dimensions(&mut self, dimensions: UVec2) {
        self.dimensions = dimensions;
    }

    fn set_background(&mut self, color: Color) {
        self.background = color;
    }

    fn scale(&mut self, factor: u32) {
        self.dimensions.x *= factor;
        self.dimensions.y *= factor;
    }
}

//TODO : render called from draw + loading screen in update with optional Image...
// impl Drawable for View {
//
//     fn draw(&self, position_in_screen: IVec2) {
//
//
//     }
//
//     fn update(&mut self, image: &Image) {
//         todo!()
//     }
// }
