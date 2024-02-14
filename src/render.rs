use macroquad::color::{RED, WHITE, YELLOW};
use macroquad::prelude::{
    clear_background, draw_texture, get_frame_time, next_frame, Image, Texture2D,
};
use macroquad::time::get_fps;
use std::time::Duration;

pub trait Renderable {
    fn render(&mut self) -> &Image;
}

///This is part of a root engine "display" module... TODO
/// Render manages only texture rendering on screen (with Z order (TODO) )
/// Image Rendering is done elsewhere.
pub struct RenderBuffer {
    texture: Texture2D, // TODO Extend this in std::collections::HashMap to get z order
    pub(crate) target_fps: i32,
}

impl RenderBuffer {
    //Note : u16 here to render in the image
    pub fn new(initial_image: &Image, target_fps: i32) -> RenderBuffer {
        // let img = Image::gen_image_color(screen_width, screen_height, WHITE);
        let txtr = Texture2D::from_image(initial_image);
        RenderBuffer {
            texture: txtr,
            target_fps,
        }
    }

    //TODO : show / hide methods to add / remove something from the list of things to render...
    // pub fn add(&mut self,) {
    //     self.renderable = Box::new(r);
    // }
    // OR : pass it into the run function only...
    // OR: a way to "link" image and texture...

    pub(crate) async fn update(&mut self, image: &Image) {
        clear_background(RED);

        self.texture.update(image);

        draw_texture(&self.texture, 0., 0., YELLOW);

        //TODO : on screen / window instead of log...
        // println!("FPS: {}", self.current_fps());

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
