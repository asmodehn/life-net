use crate::graphics::sprite::Sprite;
use macroquad::color::RED;
use macroquad::math::IVec2;
use macroquad::prelude::{clear_background, get_fps, get_frame_time, next_frame};
use macroquad::ui;
use std::ops::Deref;
use std::time::Duration;

use crate::graphics::quad::{Drawable, Updatable};
use crate::graphics::Viewable;

pub struct View {
    sprite: Sprite, // TODO Extend this in std::collections::HashMap to get z order
    pub(crate) target_fps: i32,
}

impl View {
    pub fn new(viewable: &impl Viewable, target_fps: i32) -> Self {
        let initial_image = viewable.render().borrow();

        let sprite = Sprite::from_image(initial_image.deref());
        Self { sprite, target_fps }
    }

    //TODO : show / hide methods to add / remove something from the list of things to render...
    // pub fn add(&mut self,) {
    //     self.renderable = Box::new(r);
    // }
    // OR : pass it into the run function only...
    // OR: a way to "link" image and texture...

    pub(crate) async fn update(&mut self, viewable: &impl Viewable) {
        clear_background(RED);

        self.sprite.update(viewable.render().borrow().deref());

        let pos = IVec2::new(0, 0);

        Drawable::draw(&self.sprite, pos);

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
