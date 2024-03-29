use crate::cell;
use figment::compute::Computable;
use figment::graphics::Viewable;
use grid::Grid;
use itertools::iproduct;
use macroquad::color::Color;
use macroquad::prelude::Image;
use macroquad::rand::ChooseRandom;
use std::cell::RefCell;
use std::iter::Peekable;
use std::ops::DerefMut;
use std::time::Duration;

pub struct QuadUpdate {
    original: Grid<cell::State>,
    left_over: Vec<(usize, usize)>,
}

impl QuadUpdate {
    pub fn new(cells: &Grid<cell::State>) -> Self {
        let original = cells.clone(); // because we need to own our copy for later compute

        //TODO : separate to not always require reshuffling... only if/when we have time...
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

//TODO : EXactSizedIterator
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
    image: RefCell<Image>,
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
            image: RefCell::new(img),
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

        Self { progress, ..self }
    }

    /// Attempt an update step.
    /// Returns false if the iterator has ended.
    fn update_step(&mut self, _elapsed: Duration, remainder: &mut Peekable<QuadUpdate>) -> bool {
        //attempt an update step
        match remainder.next() {
            None => false,
            Some((_, _, None)) => true, // out of bounds ?
            Some((x, y, Some(cell_state))) => {
                self.progress[(x as usize, y as usize)] = cell_state;
                true
            }
        }
    }

    pub(crate) fn stepper(&self) -> QuadUpdate {
        QuadUpdate::new(&self.progress)
    }
}

impl Computable for Quad {
    type Stepper = QuadUpdate;

    fn compute_reset(&self) -> Peekable<QuadUpdate> {
        self.stepper().peekable()
    }

    fn compute(&mut self, _elapsed: Duration, remainder: &mut Peekable<QuadUpdate>) {
        while self.update_step(_elapsed, remainder) {
            //noop
        }
    }

    fn compute_until(
        &mut self,
        _elapsed: Duration,
        remainder: &mut Peekable<QuadUpdate>,
        until: impl Fn() -> bool,
    ) {
        while self.update_step(_elapsed, remainder) {
            if until() {
                break;
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
    use crate::cell::State;
    use crate::quad::Quad;
    use std::time::Duration;

    use figment::compute::Computable;

    use grid::grid;
    use test::Bencher;

    #[test]
    fn cell_dies_alone() {
        let mut q = Quad::new(grid![[cell::State::Alive]]);

        let mut stpr = q.compute_reset();
        //one update
        q.compute(Duration::new(0, 0), &mut stpr);

        assert_eq!(q.progress[(0, 0)], cell::State::Dead)
    }

    #[test]
    fn cell_dies_alone_minimal_update() {
        let mut q = Quad::new(grid![[cell::State::Alive]]);

        let mut stpr = q.compute_reset();
        let _ = q.compute_until(Duration::new(0, 0), &mut stpr, || true);

        assert_eq!(q.progress[(0, 0)], cell::State::Dead)
    }

    #[test]
    fn check_stationary_squad() {
        //permanent square in quad
        let mut q = Quad::new(
            grid![[cell::State::Alive, cell::State::Alive][cell::State::Alive, cell::State::Alive]],
        );

        let mut stepper = q.compute_reset();
        //one update
        q.compute(Duration::new(0, 0), &mut stepper);

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

        let mut stepper = q.compute_reset();
        //one step only
        let _ = q.compute_until(Duration::new(0, 0), &mut stepper, || true);

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
            let mut stepper = q.compute_reset();
            q.compute(Duration::new(0, 0), &mut stepper);
        });
    }

    #[bench]
    fn bench_update_128_128(b: &mut Bencher) {
        let mut q = Quad::gen(State::Dead, 128, 128).with_random_cells();

        b.iter(|| {
            let mut stepper = q.compute_reset();
            q.compute(Duration::new(0, 0), &mut stepper);
        });
    }

    #[bench]
    fn bench_update_256_256(b: &mut Bencher) {
        let mut q = Quad::gen(State::Dead, 256, 256).with_random_cells();

        b.iter(|| {
            let mut stepper = q.compute_reset();
            q.compute(Duration::new(0, 0), &mut stepper);
        });
    }
}
