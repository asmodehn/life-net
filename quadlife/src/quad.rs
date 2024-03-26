use figment::compute::{Computable, PartialComputable};
use figment::graphics::Viewable;
use crate::cell;
use grid::Grid;
use itertools::iproduct;
use macroquad::color::Color;
use macroquad::prelude::Image;
use macroquad::rand::ChooseRandom;
use std::any::Any;
use std::cell::RefCell;
use std::iter::{Cycle, Peekable};
use std::ops::{Deref, DerefMut};
use std::time::Duration;

pub struct QuadUpdate {
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
            original,
            left_over,
        }
    }

    fn completed(&self) -> bool {
        self.left_over.is_empty()
    }
}

impl Iterator for QuadUpdate {
    type Item = (usize, usize, Option<cell::State>);

    fn next(&mut self) -> Option<Self::Item> {
        match self.left_over.pop() {
            None => None,
            Some((y, x)) => {
                let updated = cell::update(&self.original, x as i32, y as i32);
                // println!("{:?} => {:?}", self.original[y as usize *self.width as usize+ x as usize], updated);
                Some((x, y, updated))
            } //CAREFUL : grid computation here must be exactly same as image...
        }
    }
}

fn to_colors(state: &Grid<cell::State>) -> Vec<Color> {
    state.iter().map(|p| cell::color(*p)).collect()
}

pub struct Quad {
    progress: Grid<cell::State>,
    update: Option<QuadUpdate>,
    image: RefCell<Image>,
    updated_cb: fn(&mut Self),
}

impl Quad {
    pub fn new(state_grid: Grid<cell::State>) -> Self {
        let update = QuadUpdate::new(&state_grid);

        let mut img = Image::gen_image_color(
            state_grid.cols() as u16,
            state_grid.rows() as u16,
            cell::color(cell::State::Dead),
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

    pub fn gen(state: cell::State, width: u16, height: u16) -> Self {
        let progress: Grid<cell::State> = Grid::init(width as usize, height as usize, state);

        Self::new(progress)
    }

    pub fn with_random_cells(self) -> Self {
        //TODO : generator as parameter
        let mut progress: Grid<cell::State> =
            Grid::init(self.width(), self.height(), cell::State::Dead);

        for s in progress.iter_mut() {
            if macroquad::prelude::rand::gen_range(0, 5) == 0 {
                *s = cell::State::Alive
            } else {
                *s = cell::State::Dead
            }
        }

        let update = QuadUpdate::new(&progress);

        Self {
            progress,
            update: Some(update),
            ..self
        }
    }

    fn reset_update(&mut self) {
        self.update = Some(QuadUpdate::new(&self.progress));
    }

    fn update<I>(&mut self, _elapsed: Duration, until: impl Fn() -> bool, remainder: &mut I)
    where
        I: Iterator<Item = (usize, usize, Option<cell::State>)>,
    {
        loop {
            //attempt an update step
            match remainder.next() {
                None => {
                    break;
                }
                Some((_, _, None)) => {} // out of bounds ?
                Some((x, y, Some(cell_state))) => {
                    self.progress[(x as usize, y as usize)] = cell_state;
                }
            }

            //late until call to ensure some progress
            if until() {
                break;
            }
        }
    }

    pub(crate) fn stepper(&self) -> QuadUpdate {
        QuadUpdate::new(&self.progress)
    }
}

impl PartialComputable for Quad {
    type Step = (usize, usize, Option<cell::State>);
    type Stepper = QuadUpdate;

    fn compute_reset(&self) -> Peekable<QuadUpdate> {
        self.stepper().peekable()
    }

    fn compute_partial(
        &mut self,
        _elapsed: Duration,
        until: impl Fn() -> bool,
        remainder: &mut Peekable<QuadUpdate>,
    ) {
        self.update(_elapsed, until, remainder);
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
            match self.update.as_mut().unwrap().next() {
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
    use crate::cell;
    use crate::cell::{State, ALIVE, DEAD};
    use crate::quad::Quad;
    use std::time::Duration;

    use figment::compute::{Computable, PartialComputable};

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

        let mut stpr = q.compute_reset();
        let _ = q.compute_partial(Duration::new(0, 0), || true, &mut stpr);

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
        let _ = q.compute_partial(Duration::new(0, 0), || true, &mut q.compute_reset());

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
