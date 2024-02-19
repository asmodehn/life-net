use macroquad::color::{RED, YELLOW};
use macroquad::prelude::{
    clear_background, draw_texture, get_fps, get_frame_time, next_frame, Image, Texture2D,
};
use macroquad::ui;
use std::time::Duration;

// mod ui;
// mod scene;
mod quad;
mod sprite;
pub(crate) mod view;
// mod sprite;
//
// use std::cell::Cell;
// use macroquad::color::{Color, RED, YELLOW};
// use macroquad::math::IVec2;
// use macroquad::prelude::{clear_background, draw_texture, Image, next_frame};
// use macroquad::ui::Ui;
//
// use crate::graphics::scene::Scene;
//
// pub trait Renderable {
//     fn render(&mut self) -> &Image;
// }
//
//
// pub trait Displayable {
//
//     /// display some graphical element
//     /// Screen is passed to everyone has the screen configuration available
//     fn display(&self, view: &View);
//
// }
//
//
//
// //TODO : implement this with some kind of structure...
//

// pub(crate) async fn display(view: &View) {
//
//     view.render(view.scene, view.position);
//
//
//
//     // clear_background(RED);
//
//     self.texture.update(image);
//
//     draw_texture(&self.texture, 0., 0., YELLOW);
//
//     //TODO : on screen / window instead of log...
//     // println!("FPS: {}", self.current_fps());
//
//     //CAREFUL with z order !
//     ui::root_ui().label(None, format!("FPS: {}", self.current_fps()).as_str());
//
//     next_frame().await;
// }
