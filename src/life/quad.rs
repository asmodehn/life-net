use crate::compute::{Computable, PartialComputable};
use crate::graphics::Viewable;
use crate::life::cell;
use crate::life::cell::{State, ALIVE, DEAD};
use grid::{grid, Grid};
use itertools::iproduct;
use macroquad::color::Color;
use macroquad::prelude::collections::storage::get_mut;
use macroquad::prelude::Image;
use macroquad::rand::ChooseRandom;
use std::cell::{Cell, RefCell};
use std::ops::{Deref, DerefMut};
use std::time::Duration;

struct QuadUpdate {
    width: usize,
    height: usize,
    //TODO : try grid crate here ??
    original: Grid<cell::State>,
    left_over: Vec<(usize, usize)>,
}

impl QuadUpdate {
    pub fn new(cells: &Grid<cell::State>) -> Self {
        let original = cells.clone(); // because we need to own our copy for later compute

        let mut left_over: Vec<(usize, usize)> =
            iproduct!(0..cells.rows(), 0..cells.cols()).collect();
        left_over.shuffle();

        Self {
            width: cells.cols(),
            height: cells.rows(),
            original,
            left_over,
        }
    }

    //TODO : pass iterator as parameter here...
    fn step(&mut self) -> Option<(usize, usize, Option<cell::State>)> {
        match self.left_over.pop() {
            None => None,
            Some((y, x)) => {
                let updated = cell::update(&self.original, x as i32, y as i32);
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

fn to_colors(state: &Grid<State>) -> Vec<Color> {
    state.iter().map(|p| cell::color(*p)).collect()
}

pub struct Quad {
    progress: Grid<cell::State>,
    update: Option<QuadUpdate>,
    image: RefCell<Image>,
    updated_cb: fn(&mut Self),
}

impl Quad {
    pub fn new(state_grid: Grid<State>) -> Self {
        let update = QuadUpdate::new(&state_grid);

        let mut img = Image::gen_image_color(
            state_grid.cols() as u16,
            state_grid.rows() as u16,
            cell::color(State::Dead),
        );
        img.update(to_colors(&state_grid).as_slice());

        Self {
            progress: state_grid,
            update: Some(update),
            image: RefCell::new(img),
            updated_cb: Self::reset_update,
        }
    }

    pub fn width(&self) -> usize {
        self.progress.cols()
    }

    pub fn height(&self) -> usize {
        self.progress.rows()
    }

    pub fn gen(state: State, width: u16, height: u16) -> Self {
        let progress: Grid<cell::State> = Grid::init(width as usize, height as usize, state);

        Self::new(progress)
    }

    pub(crate) fn with_random_cells(self) -> Self {
        //TODO : generator as parameter
        let mut progress: Grid<cell::State> = Grid::init(self.width(), self.height(), State::Dead);

        for s in progress.iter_mut() {
            if macroquad::prelude::rand::gen_range(0, 5) == 0 {
                *s = State::Alive
            } else {
                *s = State::Dead
            }
        }

        let update = QuadUpdate::new(&progress);

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
        self.update = Some(QuadUpdate::new(&self.progress));
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
                self.update = Some(QuadUpdate::new(&self.progress));
            }

            //attempt an update step
            match self.update.as_mut().unwrap().step() {
                None => {}
                Some((_, _, None)) => {}
                Some((x, y, Some(cell_state))) => {
                    self.progress[(x as usize, y as usize)] = cell_state;
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
                self.update = Some(QuadUpdate::new(&self.progress));
            }

            //attempt an update step
            match self.update.as_mut().unwrap().step() {
                None => {}
                Some((_, _, None)) => {}
                Some((x, y, Some(cell_state))) => {
                    self.progress[(x as usize, y as usize)] = cell_state;
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

    use grid::grid;
    use test::Bencher;

    #[test]
    fn cell_dies_alone() {
        let mut q = Quad::new(grid![[cell::State::Alive]]);

        //one update
        q.compute(Duration::new(0, 0));

        assert_eq!(q.progress[(0, 0)], cell::State::Dead)
    }

    #[test]
    fn cell_dies_alone_minimal_update() {
        let mut q = Quad::new(grid![[cell::State::Alive]]);

        q.compute_partial(Duration::new(0, 0), || true);

        assert_eq!(q.progress[(0, 0)], cell::State::Dead)
    }

    #[test]
    fn check_stationary_squad() {
        //permanent square in quad
        let mut q = Quad::new(
            grid![[cell::State::Alive, cell::State::Alive][cell::State::Alive, cell::State::Alive]],
        );

        //one update
        q.compute(Duration::new(0, 0));

        assert_eq!(
            q.progress,
            grid![[cell::State::Alive, cell::State::Alive][cell::State::Alive, cell::State::Alive]]
        )
    }

    #[test]
    fn check_stationary_squad_minimal_update() {
        let mut q = Quad::new(
            grid![[cell::State::Alive, cell::State::Alive][cell::State::Alive, cell::State::Alive]],
        );
        //permanent square in quad

        //one update
        q.compute_partial(Duration::new(0, 0), || true);

        assert_eq!(
            q.progress,
            grid![[cell::State::Alive, cell::State::Alive][cell::State::Alive, cell::State::Alive]]
        )
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
