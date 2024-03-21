use crate::life::cell::State::{Alive, Dead};
use grid::Grid;
use macroquad::color; // TODO : replace with our color modules...

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum State {
    Alive,
    Dead,
}

pub const ALIVE: color::Color = color::BLACK;
pub const DEAD: color::Color = color::WHITE;

pub(crate) fn state(color: &[u8; 4]) -> State {
    if <color::Color as Into<[u8; 4]>>::into(ALIVE) == *color {
        Alive
    } else if <color::Color as Into<[u8; 4]>>::into(DEAD) == *color {
        Dead
    } else {
        Dead
    }
}

pub(crate) fn color(state: State) -> color::Color {
    match state {
        Alive => ALIVE,
        Dead => DEAD,
    }
}

fn neighbours_count(cells: &Grid<State>, x: i32, y: i32) -> i32 {
    let mut neighbors_count = 0;

    for j in -1i32..=1 {
        for i in -1i32..=1 {
            // if not the cell itself
            if i != 0 || j != 0 {
                match cells.get((x + i) as usize, (y + j) as usize) {
                    None => {} // out of bounds
                    Some(&Alive) => {
                        neighbors_count += 1;
                    } // alive neighbour
                    Some(&Dead) => {} // dead neighbour
                }
            }
        }
    }
    return neighbors_count;
}

pub fn update(cells: &Grid<State>, x: i32, y: i32) -> Option<State> {
    let neighbors_count = neighbours_count(&cells, x, y);

    let current_cell = cells.get(x, y)?;

    // Note : current rules : b3s23 -> devise a way to parameterize the life rules from startup ?
    Some(match (current_cell, neighbors_count) {
        // Rule 1: Any live cell with fewer than two live neighbours
        // dies, as if caused by underpopulation.
        (&State::Alive, x) if x < 2 => State::Dead,
        // Rule 2: Any live cell with two or three live neighbours
        // lives on to the next generation.
        (&State::Alive, 2) | (State::Alive, 3) => State::Alive,
        // Rule 3: Any live cell with more than three live
        // neighbours dies, as if by overpopulation.
        (&State::Alive, x) if x > 3 => State::Dead,
        // Rule 4: Any dead cell with exactly three live neighbours
        // becomes a live cell, as if by reproduction.
        (&State::Dead, 3) => State::Alive,
        // All other cells remain in the same state.
        (otherwise, _) => *otherwise,
    })
}

pub fn update_local(cell: &State, neighbours: [Option<&State>; 8]) -> State {
    let alive = neighbours
        .into_iter()
        .filter(|&os| os.is_some_and(|&s| s == State::Alive))
        .count();

    match (cell, alive) {
        // Rule 1: Any live cell with fewer than two live neighbours
        // dies, as if caused by underpopulation.
        (State::Alive, a) if a < 2 => State::Dead,
        // Rule 2: Any live cell with two or three live neighbours
        // lives on to the next generation.
        (State::Alive, 2) | (State::Alive, 3) => State::Alive,
        // Rule 3: Any live cell with more than three live
        // neighbours dies, as if by overpopulation.
        (State::Alive, a) if a > 3 => State::Dead,
        // Rule 4: Any dead cell with exactly three live neighbours
        // becomes a live cell, as if by reproduction.
        (State::Dead, 3) => State::Alive,
        // All other cells remain in the same state.
        (otherwise, _) => *otherwise,
    }
}

#[cfg(test)]
mod tests {
    use crate::life::cell;
    use grid::{grid, Grid};
    use test::Bencher;

    #[test]
    fn check_rule1() {
        let a = cell::State::Alive;
        let d = cell::State::Dead;
        // all possible cases with 0 or 1 neighbour (<2)
        let alone = grid![[d, d, d] [ d, a, d] [ d, d, d]];
        let nn = grid![[d, a, d] [d, a, d] [d, d, d]];
        let sn = grid![[d, d, d] [d, a, d] [d, a, d]];
        let en = grid![[d, d, d][d, a, a] [d, d, d]];
        let wn = grid![[d, d, d] [ a, a, d][ d, d, d]];
        let nen = grid![[d, d, a] [d, a, d] [d, d, d]];
        let nwn = grid![[a, d, d] [ d, a, d] [ d, d, d]];
        let sen = grid![[d, d, d] [d, a, d] [d, d, a]];
        let swn = grid![[d, d, d] [d, a, d][ a, d, d]];

        assert_eq!(cell::neighbours_count(&alone, 1, 1), 0);
        assert_eq!(cell::update(&alone, 1, 1), Some(cell::State::Dead));

        assert_eq!(cell::neighbours_count(&nn, 1, 1), 1);
        assert_eq!(cell::update(&nn, 1, 1), Some(cell::State::Dead));

        assert_eq!(cell::neighbours_count(&sn, 1, 1), 1);
        assert_eq!(cell::update(&sn, 1, 1), Some(cell::State::Dead));

        assert_eq!(cell::neighbours_count(&en, 1, 1), 1);
        assert_eq!(cell::update(&en, 1, 1), Some(cell::State::Dead));

        assert_eq!(cell::neighbours_count(&wn, 1, 1), 1);
        assert_eq!(cell::update(&wn, 1, 1), Some(cell::State::Dead));

        assert_eq!(cell::neighbours_count(&nen, 1, 1), 1);
        assert_eq!(cell::update(&nen, 1, 1), Some(cell::State::Dead));

        assert_eq!(cell::neighbours_count(&nwn, 1, 1), 1);
        assert_eq!(cell::update(&nwn, 1, 1), Some(cell::State::Dead));

        assert_eq!(cell::neighbours_count(&sen, 1, 1), 1);
        assert_eq!(cell::update(&sen, 1, 1), Some(cell::State::Dead));

        assert_eq!(cell::neighbours_count(&swn, 1, 1), 1);
        assert_eq!(cell::update(&swn, 1, 1), Some(cell::State::Dead));
    }

    #[test]
    fn check_rule3() {
        let a = cell::State::Alive;
        let d = cell::State::Dead;
        // all possible cases with 4, 5, 6, 7 or 8 neighbours (>3)
        let surrounded = grid![[a, a, a] [ a, a, a][ a, a, a]];
        let seven: Vec<Grid<cell::State>> = vec![
            grid![[d, a, a] [ a, a, a][ a, a, a]],
            grid![[a, d, a][ a, a, a][a, a, a]],
            grid![[a, a, d][ a, a, a][ a, a, a]],
            grid![[a, a, a][ d, a, a ][a, a, a]],
            grid![[a, a, a][ a, a, d][ a, a, a]],
            grid![[a, a, a][ a, a, a][ d, a, a]],
            grid![[a, a, a][ a, a, a][ a, d, a]],
            grid![[a, a, a][a, a, a][ a, a, d]],
        ];
        //TODO : combinations for 6, 5 and 4...

        assert_eq!(cell::neighbours_count(&surrounded, 1, 1), 8);
        assert_eq!(cell::update(&surrounded, 1, 1), Some(cell::State::Dead));
        for s in seven {
            assert_eq!(cell::neighbours_count(&s, 1, 1), 7);
            assert_eq!(cell::update(&s, 1, 1), Some(cell::State::Dead));
        }
    }

    #[test]
    fn check_rule2() {
        let a = cell::State::Alive;
        let d = cell::State::Dead;

        //TODO : all combinations for 2 and 3...
        let two = grid![[d, a, a][ d, a, d][ d, d, d]];
        let three = grid![[d, d, a][ d, a, a][a, d, d]];

        assert_eq!(cell::neighbours_count(&two, 1, 1), 2);
        assert_eq!(cell::update(&two, 1, 1), Some(cell::State::Alive));

        assert_eq!(cell::neighbours_count(&three, 1, 1), 3);
        assert_eq!(cell::update(&three, 1, 1), Some(cell::State::Alive));
    }

    #[test]
    fn check_rule4() {
        let a = cell::State::Alive;
        let d = cell::State::Dead;

        let three = grid![[d, d, a][ d, d, a][ a, d, d]];

        assert_eq!(cell::neighbours_count(&three, 1, 1), 3);
        assert_eq!(cell::update(&three, 1, 1), Some(cell::State::Alive));
    }

    #[test]
    fn stay_dead_with_two_neighbours() {
        let a = cell::State::Alive;
        let d = cell::State::Dead;

        let two = grid![[d, d, a][ d, d, d][a, d, d]];

        assert_eq!(cell::neighbours_count(&two, 1, 1), 2);
        assert_eq!(cell::update(&two, 1, 1), Some(cell::State::Dead));
    }

    #[test]
    fn stay_dead_with_four_neighbours() {
        let a = cell::State::Alive;
        let d = cell::State::Dead;

        let four = grid![[d, d, a][d, d, a][a, d, a]];

        assert_eq!(cell::neighbours_count(&four, 1, 1), 4);
        assert_eq!(cell::update(&four, 1, 1), Some(cell::State::Dead));
    }

    #[bench]
    fn bench_update(b: &mut Bencher) {
        let a = cell::State::Alive;
        let d = cell::State::Dead;

        let w = grid![[d, d, a][d, d, a][a, d, a]];

        b.iter(|| cell::update(&w, 1, 1));
    }
}
