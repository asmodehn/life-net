use crate::graphics::scene::Scene;
use crate::graphics::sprite::Sprite;
use crate::graphics::view::View;
use macroquad::color::{RED, YELLOW};
use macroquad::math::IVec2;
use macroquad::miniquad::CursorIcon::Default;
use macroquad::prelude::{
    clear_background, draw_texture, get_fps, get_frame_time, next_frame, screen_height,
    screen_width, Image, Texture2D,
};
use macroquad::ui;
use std::time::Duration;

// mod ui;
pub(crate) mod quad;
pub(crate) mod scene;
pub(crate) mod sprite;
pub(crate) mod view;

//Maybe default scene is a baad idea...
// const DEFAULT_SCENE: Scene = Default::default();
//
// async fn display() {
//     if !DEFAULT_SCENE.has_view() {
//
//         let width = screen_width().floor() as u32;
//         let height = screen_height().floor() as u32;
//
//         let v = View::new(&DEFAULT_SCENE, IVec2::new(0,0))
//             .with_dimensions(width, height);
//         DEFAULT_SCENE.add_view(v);
//     }
//
//
//
//     DEFAULT_SCENE.display().await
// }

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
