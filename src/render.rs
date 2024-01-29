// use crate::engine::Updatable;
use crate::life::World;
use macroquad::color::WHITE;
use macroquad::prelude::{
    clear_background, draw_texture, get_frame_time, next_frame, Image, Texture2D,
};
use std::time::Duration;

pub trait Renderable {
    fn render(&mut self) -> &Image;
}

pub struct RenderBuffer {
    texture: Texture2D,
}
//TODO : rename this Engine -> Buffer

impl RenderBuffer {
    //Note : u16 here to render in the image
    pub fn new(initial_image: &Image) -> RenderBuffer {
        // let img = Image::gen_image_color(screen_width, screen_height, WHITE);
        let txtr = Texture2D::from_image(initial_image);
        RenderBuffer { texture: txtr }
    }

    //TODO : show / hide methods to add / remove something from the list of things to render...
    // pub fn add(&mut self,) {
    //     self.renderable = Box::new(r);
    // }
    // OR : pass it into the run function only...

    fn update(&mut self, image: &Image) {
        clear_background(WHITE);

        self.texture.update(image);

        draw_texture(&self.texture, 0., 0., WHITE);
    }
}

//TODO : extend this -> Engine with async loop and refresh rate management...
pub async fn run(rb: &mut RenderBuffer, world: &mut World) {
    loop {
        //since render module focuses on rendering only
        // the time spent here is the frametime returned by macroquad
        let elapsed = Duration::from_secs_f32(get_frame_time());

        //TMP render update driving simulation update
        world.update(elapsed);
        //TODO : pass a call (closure?) to life::run() as argument here.

        //TMP : render here instead of update ? because we want to avoid hold world ref in renderbuffer ?
        let img = world.render();

        rb.update(img);

        next_frame().await;
    }
}
