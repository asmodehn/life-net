use crate::compute::{Computable, PartialComputable};
use crate::graphics::Viewable;
use crate::life::cell;
use crate::life::cell::State;
use grid::{Grid, Order};
use itertools::iproduct;
use macroquad::color::Color;
use macroquad::prelude::Image;
use macroquad::rand::ChooseRandom;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::iter::{Enumerate, Map};
use std::ops::DerefMut;
use std::slice::Iter;
use std::time::Duration;

use rand::distributions::uniform::SampleBorrow;

use rand::seq::SliceRandom;
use rand::Rng;

struct QuadUpdate {
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

struct ShuffledIter<I>(Vec<I>);

impl<I> ShuffledIter<I> {
    fn new(i: impl Iterator<Item = I>) -> Self {
        let mut indexed_vec: Vec<I> = i.collect();
        indexed_vec.shuffle();

        Self(indexed_vec)
    }
}

impl<I> Iterator for ShuffledIter<I> {
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Quad {
    original: Grid<cell::State>,
    progress: Option<Grid<cell::State>>,
    update: Option<QuadUpdate>,
    image: RefCell<Image>,
    updated_cb: fn(&mut Self),
}

impl Quad {
    //TODO : optimizatin : maybe uninitialized... (check: some collections already support MaybeUninit)
    pub fn new(state_grid: Grid<State>) -> Self {
        let update = QuadUpdate::new(&state_grid);

        let mut img = Image::gen_image_color(
            state_grid.cols() as u16,
            state_grid.rows() as u16,
            cell::color(State::Dead),
        );
        img.update(to_colors(&state_grid).as_slice());

        Self {
            original: state_grid,
            progress: None,
            update: Some(update),
            image: RefCell::new(img),
            updated_cb: Self::reset_update,
        }
    }

    pub fn width(&self) -> usize {
        self.original.cols()
    }

    pub fn height(&self) -> usize {
        self.original.rows()
    }

    pub fn gen(state: State, width: u16, height: u16) -> Self {
        let g: Grid<cell::State> = Grid::init(width as usize, height as usize, state);

        Self::new(g)
    }

    pub(crate) fn with_random_cells(self) -> Self {
        //TODO : generator as parameter
        let mut g: Grid<cell::State> = Grid::init(self.width(), self.height(), State::Dead);

        for s in g.iter_mut() {
            if macroquad::prelude::rand::gen_range(0, 5) == 0 {
                *s = State::Alive
            } else {
                *s = State::Dead
            }
        }

        let update = QuadUpdate::new(&g);

        Self {
            original: g,
            progress: None,
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
        self.update = Some(QuadUpdate::new(&self.original));
    }

    fn indexed_iter<'a>(
        &self,
    ) -> Map<Enumerate<Iter<'a, State>>, fn((usize, &'a State)) -> ((usize, usize), &'a State)>
// where I: Iterator<Item=((usize, usize), &'a cell::State)>
    {
        // let it = self.original.indexed_iter();
        // QuadIter::<'s>::new(self)
        //
        // GridIter::<'s, cell::State, I>::new(&self.original)

        //Reimplementing indexed_iter with type parameter (to make wrapping simpler)
        self.original.iter().enumerate().map(move |(idx, i)| {
            let position = match self.original.order() {
                Order::RowMajor => (idx / self.original.cols(), idx % self.original.cols()),
                Order::ColumnMajor => (idx % self.original.rows(), idx / self.original.rows()),
            };
            (position, i)
        })
    }

    // fn indexed_shuffled_iter<'s, I>(&'s self) -> QuadIter<'s, I>
    // where  I: Iterator<Item = ((usize, usize), &'s cell::State)>
    // {
    //
    //     // let mut v: Vec<((usize, usize), &'s cell::State)> = self.original.indexed_iter().collect();
    //     // v.shuffle();
    //
    //     // let it = v.into_iter();
    //     QuadIter::new(self)
    //
    // }
}

// struct GridIter<'g, T, I>
// where I: Iterator<Item=((usize,usize), &T)>{
//     grid: &'g Grid<T>,
//     iter: I
// }
//
// impl<'g, T, I> GridIter<'g, T, I>
//     where I: Iterator<Item=((usize,usize), &T)>
// {
//     fn new(grid: &'g Grid<T>) -> Self {
//         GridIter {
//             grid: grid,
//             iter: grid.indexed_iter()
//         }
//     }
// }
//
//
// impl<'g, T, I> Iterator for GridIter<'g, T, I>
// where I: Iterator<Item=((usize,usize), &T)>
// {
//     type Item = ((usize, usize), &'g T);
//
//     fn next(&mut self) -> Option<Self::Item> {
//         self.iter.next()
//     }
//
//
// }

//
//
// struct QuadIter<'q, I: Iterator>{
//     quad: &'q Quad,
//     iter: I
// }
//
// impl<'q, I: Iterator<Item = ((usize, usize), &'q cell::State)> > QuadIter<'q, I>
// {
//     fn new(q: &'q Quad) -> Self {
//         Self{
//             quad: q,
//             iter: q.original.indexed_iter()
//         }
//     }
//
// }
//
//
// impl<'cell, I: Iterator<Item = ((usize, usize), &'cell cell::State)>> Iterator for QuadIter<'cell, I>
// {
//     type Item = ((usize, usize), (&'cell cell::State, [Option<&'cell cell::State>; 8]));
//
//     fn next(&mut self) -> Option<Self::Item> {
//         match self.iter.next() {
//             None => None,
//             Some(((r, c), s)) => {
//                 let neighbours = [
//                     self.quad.original.get(r-1, c-1), self.quad.original.get(r-1, c), self.quad.original.get(r-1, c+1),
//                     self.quad.original.get(r, c-1),                                            self.quad.original.get(r, c+1),
//                     self.quad.original.get(r+1, c-1), self.quad.original.get(r+1, c), self.quad.original.get(r+1, c+1)
//                 ];
//                 Some(((r, c), (s, neighbours)))
//             }
//         }
//     }
// }

impl PartialComputable for Quad {
    type Idx = (usize, usize);
    type El = (&'p cell::State, [Option<&'p cell::State>; 8]);

    fn compute_partial<'p, 'c, R>(
        &mut self,
        _elapsed: Duration,
        until: impl Fn() -> bool,
        remaining: Option<R>,
    ) -> Option<R>
    where
        R: Iterator<Item = (Self::Idx, Self::El)>,
    {
        let mut it: R = if remaining.is_none() {
            self.indexed_iter() // TODO : shuffle
        } else {
            remaining.expect("progress has to be initialized here")
        };

        //TODO : copy the quad because it will be changed, and pass it with remaining param...

        loop {
            match it.next() {
                Some(((r, c), (s, neighbours))) => {
                    let u = cell::update_local(&s, neighbours);

                    self.original
                        .get_mut(r, c)
                        .and_then(|ps| Some(std::mem::replace(ps, u)));
                }

                //we re done
                None => {
                    // std::mem::swap(self.original.borrow_mut(), self.progress.as_mut().unwrap());
                    break;
                }
            }

            //late until to ensure some progress
            if until() {
                // return iterator for left over cells
                break;
            }
        }

        Some(it)
    }

    fn update_completed(&self) -> bool {
        self.update.as_ref().is_some_and(|iup| iup.completed())
    }
}

impl Computable for Quad {
    fn compute(&mut self, _elapsed: Duration) {
        self.progress = Some(Grid::init(
            self.original.rows(),
            self.original.cols(),
            cell::State::Dead,
        ));

        for ((row, col), _) in self.original.indexed_iter() {
            if let Some(s) = cell::update(&self.original, row as i32, col as i32) {
                self.progress
                    .as_mut()
                    .unwrap()
                    .get_mut(row, col)
                    .and_then(|ps| Some(std::mem::replace(ps, s)));
            }
        }

        std::mem::swap(self.original.borrow_mut(), self.progress.as_mut().unwrap());
        self.progress = None;
    }
}

impl Viewable for Quad {
    fn render(&self) -> &RefCell<Image> {
        if self.progress.is_some() {
            self.image
                .borrow_mut()
                .deref_mut()
                .update(to_colors(self.progress.as_ref().unwrap()).as_slice());
        }
        &self.image
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::life::cell;
//     use crate::life::cell::{State, ALIVE, DEAD};
//     use crate::life::quad::{Quad, QuadIter};
//     use std::time::Duration;
//
//     use crate::compute::{Computable, ComputeCtx, PartialComputable};
//
//     use grid::grid;
//     use test::Bencher;
//     use crate::life;
//
//     #[test]
//     fn cell_dies_alone() {
//         let mut q = Quad::new(grid![[cell::State::Alive]]);
//
//         //one update
//         q.compute(Duration::new(0, 0));
//
//         assert_eq!(q.original[(0, 0)], cell::State::Dead)
//     }
//
//     #[test]
//     fn cell_dies_alone_minimal_update() {
//         let mut q = Quad::new(grid![[cell::State::Alive]]);
//
//         q.compute_partial(Duration::new(0, 0), || true, None);
//
//         //TODO : verify original didnt change
//         assert_eq!(q.progress.unwrap()[(0, 0)], cell::State::Dead)
//     }
//
//     #[test]
//     fn check_stationary_squad() {
//         //permanent square in quad
//         let mut q = Quad::new(
//             grid![[cell::State::Alive, cell::State::Alive][cell::State::Alive, cell::State::Alive]],
//         );
//
//         //one update
//         q.compute(Duration::new(0, 0));
//
//         assert_eq!(
//             q.original,
//             grid![[cell::State::Alive, cell::State::Alive][cell::State::Alive, cell::State::Alive]]
//         )
//     }
//
//     #[test]
//     fn check_stationary_squad_minimal_update() {
//         let mut q = Quad::new(
//             grid![[cell::State::Alive, cell::State::Alive][cell::State::Alive, cell::State::Alive]],
//         );
//         //permanent square in quad
//
//         //one update
//         q.compute_partial(Duration::new(0, 0), || true, None);
//
//         //TODO : verify original didnt change
//         assert_eq!(
//             q.progress.unwrap(),
//             grid![[cell::State::Alive, cell::State::Alive][cell::State::Alive, cell::State::Alive]]
//         )
//     }
//
//     // TODO : check blinking !
//
//     #[bench]
//     fn bench_update_064_064(b: &mut Bencher) {
//         let mut q = Quad::gen(State::Dead, 64, 64).with_random_cells();
//
//         b.iter(|| {
//             q.compute(Duration::new(0, 0));
//         });
//     }
//
//     #[bench]
//     fn bench_update_128_128(b: &mut Bencher) {
//         let mut q = Quad::gen(State::Dead, 128, 128).with_random_cells();
//
//         b.iter(|| {
//             q.compute(Duration::new(0, 0));
//         });
//     }
//
//     #[bench]
//     fn bench_update_256_256(b: &mut Bencher) {
//         let mut q = Quad::gen(State::Dead, 256, 256).with_random_cells();
//
//         b.iter(|| {
//             q.compute(Duration::new(0, 0));
//         });
//     }
// }
