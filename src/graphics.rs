use crate::graphics::quad::{Drawable, Updatable};
use crate::graphics::view::Viewable;
use macroquad::color::{RED, YELLOW};
use macroquad::math::IVec2;
use macroquad::prelude::{
    clear_background, draw_texture, get_fps, get_frame_time, next_frame, Color, Image, Texture2D,
};
use macroquad::ui;
use std::time::Duration;

// mod ui;
// mod scene;
pub(crate) mod quad;
mod rect;
pub(crate) mod sprite;
mod texture;
pub(crate) mod view;

const DEFAULT_BACKGROUND: Color = RED;

pub fn last_frame_time() -> Duration {
    Duration::from_secs_f32(get_frame_time())
}

#[allow(dead_code)]
pub fn current_fps() -> i32 {
    get_fps()
    //TODO : average these over time... in update ? only when used ?
}

pub(crate) fn target_frame_time(target_fps: f32) -> Duration {
    Duration::from_secs_f32(1. / target_fps)
}

pub(crate) fn update(d: &mut impl Updatable, v: &impl Viewable) {
    d.update(v.render());
}

pub(crate) async fn render(d: &impl Drawable, pos: IVec2) {
    //
    // pub(crate) async fn update(&mut self, viewable: &mut impl Drawable) {
    clear_background(DEFAULT_BACKGROUND);

    d.draw(pos);

    //TODO : on screen / window instead of log...
    // println!("FPS: {}", self.current_fps());

    //CAREFUL with z order !
    ui::root_ui().label(None, format!("FPS: {}", current_fps()).as_str());

    next_frame().await;
}
