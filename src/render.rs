use crate::engine::Updatable;
use macroquad::color::WHITE;
use macroquad::prelude::{
    clear_background, draw_texture, get_frame_time, next_frame, Image, Texture2D,
};
use std::time::Duration;

pub trait Renderable: Updatable {
    fn render<'s>(&'s self, image: &'s mut Image) -> &'s Image;
}

pub struct RenderEngine<'r> {
    renderable: &'r mut dyn Renderable,
    pub image: Image,
    texture: Texture2D,
}
//TODO : extend this -> RenderEngine with async loop and refresh rate management...

impl RenderEngine<'_> {
    pub fn new(screen_width: usize, screen_height: usize, r: &mut impl Renderable) -> RenderEngine {
        let img = Image::gen_image_color(screen_width as u16, screen_height as u16, WHITE);
        let txtr = Texture2D::from_image(&img);
        RenderEngine {
            renderable: r,
            image: img,
            texture: txtr,
        }
    }

    //TODO : show / hide methods to add / remove something from the list of things to render...
    // pub fn add(&mut self,) {
    //     self.renderable = Box::new(r);
    // }
    // OR : pass it into the run function only...
}

impl Updatable for RenderEngine<'_> {
    fn update(&mut self, elapsed: Duration) {
        clear_background(WHITE);

        //TMP render update driving simulatio update
        self.renderable.update(elapsed);

        self.renderable.render(&mut self.image);

        self.texture.update(&self.image);

        draw_texture(&self.texture, 0., 0., WHITE);
    }
}

// impl Engine for RenderEngine{
//     fn run_once(&mut self, world: &mut (impl Updatable)){
//
//
//         //TODO : separate update refresh rate and render rate...
//         world.update();
//
//         self.render(world);
//
//     }
//     async fn run (&mut self, world: &mut (impl Updatable)){
//         loop{
//             self.run_once(world);
//
//             next_frame().await;
//         }
//     }
// }

pub async fn run(r: &mut RenderEngine<'_>) {
    loop {
        //since render module focuses on rendering only
        // the time spent here is the frametime returned by macroquad
        r.update(Duration::from_secs_f32(get_frame_time()));

        next_frame().await;
    }
}
