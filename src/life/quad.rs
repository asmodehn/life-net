use crate::compute::{Computable, PartialComputable};
use crate::graphics::Viewable;
use crate::life::cell;
use crate::life::cell::{State, ALIVE, DEAD};
use itertools::iproduct;
use macroquad::color::Color;
use macroquad::prelude::Image;
use macroquad::rand::ChooseRandom;
use std::cell::{Cell, RefCell};
use std::ops::{Deref, DerefMut};
use std::time::Duration;

struct QuadUpdate {
    width: u16,
    height: u16,
    //TODO : try grid crate here ??
    original: Vec<cell::State>,
    left_over: Vec<(u16, u16)>,
}

impl QuadUpdate {
    pub fn new(cells: &Vec<cell::State>, width: u16, height: u16) -> Self {
        let original = cells.clone(); // because we need to own our copy for later compute

        let mut left_over: Vec<(u16, u16)> = iproduct!(0..height, 0..width).collect();
        left_over.shuffle();

        Self {
            width,
            height,
            original,
            left_over,
        }
    }

    //TODO : pass iterator as parameter here...
    fn step(&mut self) -> Option<(u16, u16, cell::State)> {
        match self.left_over.pop() {
            None => None,
            Some((y, x)) => {
                let updated =
                    cell::update(&self.original, x as i32, y as i32, self.width, self.height);
                // println!("{:?} => {:?}", self.original[y as usize *self.width as usize+ x as usize], updated);
                Some((x, y, updated))
            } //CAREFUL : grid computation here must be exactly same as image...
        }
    }
    // Note : This is an iterator. TODO : replace by an usual iterator implementation... (after grid replacement)

    fn completed(&self) -> bool {
        self.left_over.is_empty()
    }
}

fn to_colors(state: &Vec<State>) -> Vec<Color> {
    state.iter().map(|p| cell::color(*p)).collect()
}

pub struct Quad {
    width: u16,
    height: u16,
    progress: Vec<cell::State>,
    update: Option<QuadUpdate>,
    image: RefCell<Image>,
    updated_cb: fn(&mut Self),
}

impl Quad {
    pub fn new(state_vec: Vec<State>, width: u16, height: u16) -> Self {
        let update = QuadUpdate::new(&state_vec, width, height);

        let mut img = Image::gen_image_color(width, height, cell::color(State::Dead));
        img.update(to_colors(&state_vec).as_slice());

        Self {
            width,
            height,
            progress: state_vec,
            update: Some(update),
            image: RefCell::new(img),
            updated_cb: Self::reset_update,
        }
    }

    pub fn gen(state: State, width: u16, height: u16) -> Self {
        let progress: Vec<cell::State> = vec![state; width as usize * height as usize];

        Self::new(progress, width, height)
    }

    pub(crate) fn with_random_cells(self) -> Self {
        //TODO : generator as parameter
        let progress: Vec<cell::State> =
            vec![State::Dead; self.width as usize * self.height as usize]
                .iter_mut()
                .map(|_| -> State {
                    if macroquad::prelude::rand::gen_range(0, 5) == 0 {
                        State::Alive
                    } else {
                        State::Dead
                    }
                })
                .collect();

        let update = QuadUpdate::new(&progress, self.width, self.height);

        Self {
            progress,
            update: Some(update),
            ..self
        }
    }

    fn with_updated_cb(self, cb: fn(&mut Self)) -> Self {
        Self {
            updated_cb: cb,
            ..self
        }
    }

    fn reset_update(&mut self) {
        self.update = Some(QuadUpdate::new(&self.progress, self.width, self.height));
    }
}

impl PartialComputable for Quad {
    fn compute_partial(&mut self, _elapsed: Duration, until: impl Fn() -> bool) {
        loop {
            //check for completion
            if self.update_completed() {
                (self.updated_cb)(self);
                break;
            }
            //TODO : test: isn't this duplication ??
            //(re)initialize if needed
            if self.update.is_none() {
                //start new update when necessary
                self.update = Some(QuadUpdate::new(&self.progress, self.width, self.height));
            }

            //attempt an update step
            match self.update.as_mut().unwrap().step() {
                None => {}
                Some((x, y, cell_state)) => {
                    self.progress[(y * self.width + x) as usize] = cell_state;
                }
            }

            //late until to ensure some progress
            if until() {
                break;
            }
        }
    }

    fn update_completed(&self) -> bool {
        self.update.as_ref().is_some_and(|iup| iup.completed())
    }
}

impl Computable for Quad {
    fn compute(&mut self, _elapsed: Duration) {
        loop {
            //check for completion
            if self.update_completed() {
                (self.updated_cb)(self);
                break;
            }
            //TODO : test: isn't this duplication ??
            //(re)initialize if needed
            if self.update.is_none() {
                //start new update when necessary
                self.update = Some(QuadUpdate::new(&self.progress, self.width, self.height));
            }

            //attempt an update step
            match self.update.as_mut().unwrap().step() {
                None => {}
                Some((x, y, cell_state)) => {
                    self.progress[(y * self.width + x) as usize] = cell_state;
                }
            }
        }
    }
}

impl Viewable for Quad {
    fn render(&self) -> &RefCell<Image> {
        self.image
            .borrow_mut()
            .deref_mut()
            .update(to_colors(&self.progress).as_slice());
        &self.image
    }
}

#[cfg(test)]
mod tests {
    use crate::life::cell;
    use crate::life::cell::{State, ALIVE, DEAD};
    use crate::life::quad::Quad;
    use std::time::Duration;

    use crate::compute::{Computable, PartialComputable};

    use test::Bencher;

    #[test]
    fn cell_dies_alone() {
        let mut q = Quad::new(vec![cell::State::Alive], 1, 1);

        //one update
        q.compute(Duration::new(0, 0));

        assert_eq!(q.progress[0], cell::State::Dead)
    }

    #[test]
    fn cell_dies_alone_minimal_update() {
        let mut q = Quad::new(vec![cell::State::Alive], 1, 1);

        q.compute_partial(Duration::new(0, 0), || true);

        assert_eq!(q.progress[0], cell::State::Dead)
    }

    #[test]
    fn check_stationary_squad() {
        //permanent square in quad
        let mut q = Quad::new(vec![cell::State::Alive; 4], 2, 2);

        //one update
        q.compute(Duration::new(0, 0));

        assert_eq!(q.progress, vec![cell::State::Alive; 4])
    }

    #[test]
    fn check_stationary_squad_minimal_update() {
        let mut q = Quad::new(vec![cell::State::Alive; 4], 2, 2);
        //permanent square in quad

        //one update
        q.compute_partial(Duration::new(0, 0), || true);

        assert_eq!(q.progress, vec![cell::State::Alive; 4])
    }

    // TODO : check blinking !

    #[bench]
    fn bench_update_064_064(b: &mut Bencher) {
        let mut q = Quad::gen(State::Dead, 64, 64).with_random_cells();

        b.iter(|| {
            q.compute(Duration::new(0, 0));
        });
    }

    #[bench]
    fn bench_update_128_128(b: &mut Bencher) {
        let mut q = Quad::gen(State::Dead, 128, 128).with_random_cells();

        b.iter(|| {
            q.compute(Duration::new(0, 0));
        });
    }

    #[bench]
    fn bench_update_256_256(b: &mut Bencher) {
        let mut q = Quad::gen(State::Dead, 256, 256).with_random_cells();

        b.iter(|| {
            q.compute(Duration::new(0, 0));
        });
    }
}
