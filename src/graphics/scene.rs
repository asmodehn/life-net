use crate::graphics::quad::{Drawable, Placed, Quad};
use crate::graphics::view::{View, Viewable};
use itertools::Position;
use macroquad::math::UVec2;
use macroquad::prelude::{Color, IVec2, IVec3, Image, Texture2D, RED};
use std::cell::{Cell, Ref, RefCell};
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};
use std::ops::{Add, AddAssign};
use std::result::Iter;

use delegate::delegate;

// use std::collections::{BTreeMap, HashMap};
// use itertools::Itertools;
// use macroquad::color::Color;
// use macroquad::math::BVec3;
// use macroquad::prelude::{draw_texture, Image, IVec2, IVec3, Vec3};
// use macroquad::texture::Texture2D;
// use macroquad::ui::widgets::Texture;
// use crate::graphics::{Displayable, View};
// use unique_id::Generator;
// use unique_id::sequence::SequenceGenerator;
use crate::graphics::sprite::Sprite;

// use crate::life::quad::Quad;
//
//
//
const DEFAULT_BACKGROUND: Color = RED;

const DEFAULT_POSITION: IVec2 = IVec2::new(0, 0);

// #[derive(Clone)]

pub(crate) struct PlacedSprite<V: Viewable> {
    position: IVec2,
    sprite: Sprite<V>,
}

impl<V: Viewable> PlacedSprite<V> {
    pub fn new(sprite: Sprite<V>) -> Self {
        Self {
            sprite,
            position: DEFAULT_POSITION,
        }
    }
}

impl<V: Viewable> Quad for PlacedSprite<V> {
    delegate! {
        to self.sprite {
            fn get_dimensions(&self) -> UVec2;
            fn get_background(&self) -> Color;
            fn set_dimensions(&mut self, dimensions: UVec2);
            fn set_background(&mut self, color: Color);
            fn scale(&mut self, factor: u32);
        }
    }
}

impl<V: Viewable> Placed for PlacedSprite<V> {
    fn get_position(&self) -> IVec2 {
        self.position
    }

    fn set_position(&mut self, position: IVec2) {
        self.position = position;
    }

    fn translate(&mut self, to: IVec2) {
        self.position.add_assign(to);
    }
}

impl<V: Viewable> Drawable for PlacedSprite<V> {
    delegate! {
            to self.sprite {

        fn draw(&self, position_in_screen: IVec2);
    fn update(&mut self);
            }
        }
}

pub(crate) struct PlacedDrawable {
    position: IVec2,
    drawable: Box<dyn Drawable>,
}

impl PlacedDrawable {
    pub fn new(drawable: Box<dyn Drawable>) -> Self {
        Self {
            drawable,
            position: DEFAULT_POSITION,
        }
    }
}

impl Drawable for PlacedDrawable {
    delegate! {
            to self.drawable {

        fn draw(&self, position_in_screen: IVec2);
    fn update(&mut self);
            }
        }
}

impl Placed for PlacedDrawable {
    fn get_position(&self) -> IVec2 {
        self.position
    }

    fn set_position(&mut self, position: IVec2) {
        self.position = position;
    }

    fn translate(&mut self, to: IVec2) {
        self.position.add_assign(to);
    }
}

#[derive(Clone, Copy)]
struct PlacedView {
    position: IVec2,
    view: View,
}

impl PlacedView {
    fn new(view: View) -> Self {
        Self {
            view,
            position: DEFAULT_POSITION,
        }
    }
}
impl Quad for PlacedView {
    delegate! {
        to self.view {
            fn get_dimensions(&self) -> UVec2;
            fn get_background(&self) -> Color;
            fn set_dimensions(&mut self, dimensions: UVec2);
            fn set_background(&mut self, color: Color);
            fn scale(&mut self, factor: u32);
        }
    }
}

impl Placed for PlacedView {
    fn get_position(&self) -> IVec2 {
        self.position
    }

    fn set_position(&mut self, position: IVec2) {
        self.position = position;
    }

    fn translate(&mut self, to: IVec2) {
        self.position.add_assign(to);
    }
}

pub(crate) struct Scene<V: Viewable> {
    pub(crate) background: Color, // ie like a skybox...
    //is this our dumb generational arena ?? WTD with those ?
    //TODO : quadtree instead ?
    sprites: Vec<RefCell<PlacedSprite<V>>>,
    drawables: Vec<RefCell<PlacedDrawable>>,
    cameras: Vec<PlacedView>, // TODO : merge in a Vec of PlacedQuads
}

impl<V: Viewable> Default for Scene<V> {
    fn default() -> Self {
        Scene {
            background: DEFAULT_BACKGROUND,
            sprites: vec![],
            drawables: vec![],
            cameras: vec![],
        }
    }
}

type SpriteId = usize;
type ViewId = usize;

impl<V: Viewable> Scene<V> {
    pub(crate) fn new(background: Color) -> Self {
        Self {
            background,
            ..Scene::default()
        }
    }

    pub(crate) fn with_sprite(self, sprite: Sprite<V>) -> Self {
        let mut s = self;
        s.add_sprite(sprite);
        s
    }

    pub(crate) fn with_drawable(self, drawable: Box<dyn Drawable>) -> Self {
        let mut s = self;
        s.add_drawable(drawable);
        s
    }

    pub(crate) fn with_view(self, view: View) -> Self {
        let mut s = self;
        s.add_view(view);
        s
    }

    pub(crate) fn has_view(&self) -> bool {
        !self.cameras.is_empty()
    }

    pub(crate) fn add_sprite(&mut self, s: Sprite<V>) -> SpriteId {
        let mut ps = PlacedSprite::new(s);
        ps.set_position(DEFAULT_POSITION);
        self.sprites.push(RefCell::new(ps));
        self.sprites.len() - 1
    }

    pub(crate) fn add_drawable(&mut self, d: Box<dyn Drawable>) -> SpriteId {
        let mut ps = PlacedDrawable::new(d);
        ps.set_position(DEFAULT_POSITION);
        self.drawables.push(RefCell::new(ps));
        self.drawables.len() - 1
    }

    pub(crate) fn add_view(&mut self, v: View) -> ViewId {
        let mut pv = PlacedView::new(v);
        pv.set_position(DEFAULT_POSITION);
        self.cameras.push(pv);
        self.cameras.len() - 1
    }

    // pub(crate) fn iter_borrow_sprites<'s>(&self) -> impl Iterator<Item=Ref<'sprite, PlacedSprite<'s>>> {
    //     self.sprites.iter().map(move |rc| rc.borrow())
    // }

    pub(crate) fn iter_borrow_drawables(&self) -> impl Iterator<Item = Ref<'_, PlacedDrawable>> {
        self.drawables.iter().map(move |rc| rc.borrow())
    }

    pub(crate) async fn display(&self) {
        //TODO : support multiple views (HOW ???)
        self.cameras[0]
            .view
            .render(self.iter_borrow_drawables())
            .await;
        //TODO : culling how ??
        //TODO : which shader, HOW ?
    }
}

//TODO : implement Iterator on Scene to iterate on quads (or cells of quads ?)...
// impl Iterator for Scene{
//     type Item = Cell<PlacedSprite>;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         self.sprites.iter().next()
//     }
// }
