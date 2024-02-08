use crate::life::cell::State::{Alive, Dead};
use crate::life::world::usize_from_i32;
use macroquad::color;
use macroquad::color_u8;
use std::{fmt, mem};

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

fn neighbours_count_on_quad(cells: &[[u8; 4]], x: i32, y: i32, w: u16, h: u16) -> i32 {
    let mut neighbors_count = 0;

    for j in -1i32..=1 {
        for i in -1i32..=1 {
            // out of bounds
            if y + j < 0 || y + j >= h as i32 || x + i < 0 || x + i >= w as i32 {
                continue;
            }
            // cell itself
            if i == 0 && j == 0 {
                continue;
            }

            let neighbor = cells[usize_from_i32((y + j) * w as i32 + (x + i))];
            if ALIVE == neighbor.into() {
                neighbors_count += 1;
            }
        }
    }
    return neighbors_count;
}

fn neighbours_count(cells: &Vec<State>, x: i32, y: i32, w: usize, h: usize) -> i32 {
    let mut neighbors_count = 0;

    for j in -1i32..=1 {
        for i in -1i32..=1 {
            // out of bounds
            if y + j < 0 || y + j >= h as i32 || x + i < 0 || x + i >= w as i32 {
                continue;
            }
            // cell itself
            if i == 0 && j == 0 {
                continue;
            }

            let neighbor = cells[usize_from_i32((y + j) * w as i32 + (x + i))];
            if neighbor == State::Alive {
                neighbors_count += 1;
            }
        }
    }
    return neighbors_count;
}

pub fn update_on_quad(cells: &[[u8; 4]], x: i32, y: i32, w: u16, h: u16) -> color::Color {
    let neighbors_count = neighbours_count_on_quad(&cells, x, y, w, h);

    let current_cell = cells[usize_from_i32(y * w as i32 + x)];

    //TODO : optimize by only taking care of case where teh cell actually changes state...
    match (state(&current_cell), neighbors_count) {
        // Rule 1: Any live cell with fewer than two live neighbours
        // dies, as if caused by underpopulation.
        (State::Alive, x) if x < 2 => DEAD,
        // Rule 2: Any live cell with two or three live neighbours
        // lives on to the next generation.
        (State::Alive, 2) | (State::Alive, 3) => ALIVE,
        // Rule 3: Any live cell with more than three live
        // neighbours dies, as if by overpopulation.
        (State::Alive, x) if x > 3 => DEAD,
        // Rule 4: Any dead cell with exactly three live neighbours
        // becomes a live cell, as if by reproduction.
        (State::Dead, 3) => ALIVE,
        // All other cells remain in the same state.
        (otherwise, _) => color(otherwise),
    }
}

pub fn update(cells: &Vec<State>, x: i32, y: i32, w: usize, h: usize) -> State {
    let neighbors_count = neighbours_count(&cells, x, y, w, h);

    let current_cell = cells[usize_from_i32(y * w as i32 + x)];

    // Note : current rules : b3s23 -> devise a way to parameterize the life rules from startup ?
    match (current_cell, neighbors_count) {
        // Rule 1: Any live cell with fewer than two live neighbours
        // dies, as if caused by underpopulation.
        (State::Alive, x) if x < 2 => State::Dead,
        // Rule 2: Any live cell with two or three live neighbours
        // lives on to the next generation.
        (State::Alive, 2) | (State::Alive, 3) => State::Alive,
        // Rule 3: Any live cell with more than three live
        // neighbours dies, as if by overpopulation.
        (State::Alive, x) if x > 3 => State::Dead,
        // Rule 4: Any dead cell with exactly three live neighbours
        // becomes a live cell, as if by reproduction.
        (State::Dead, 3) => State::Alive,
        // All other cells remain in the same state.
        (otherwise, _) => otherwise,
    }
}

#[cfg(test)]
mod tests {
    use crate::life::cell;
    use crate::life::cell::color;
    use crate::life::quad::Quad;
    use std::time::Duration;
    use test::Bencher;

    use cell::{ALIVE, DEAD};

    #[test]
    fn check_rule1() {
        let a = cell::State::Alive;
        let d = cell::State::Dead;
        // all possible cases with 0 or 1 neighbour (<2)
        let alone = vec![d, d, d, d, a, d, d, d, d];
        let nn = vec![d, a, d, d, a, d, d, d, d];
        let sn = vec![d, d, d, d, a, d, d, a, d];
        let en = vec![d, d, d, d, a, a, d, d, d];
        let wn = vec![d, d, d, a, a, d, d, d, d];
        let nen = vec![d, d, a, d, a, d, d, d, d];
        let nwn = vec![a, d, d, d, a, d, d, d, d];
        let sen = vec![d, d, d, d, a, d, d, d, a];
        let swn = vec![d, d, d, d, a, d, a, d, d];
        assert_eq!(cell::update(&alone, 1, 1, 3, 3), cell::State::Dead);
        assert_eq!(cell::update(&nn, 1, 1, 3, 3), cell::State::Dead);
        assert_eq!(cell::update(&sn, 1, 1, 3, 3), cell::State::Dead);
        assert_eq!(cell::update(&en, 1, 1, 3, 3), cell::State::Dead);
        assert_eq!(cell::update(&wn, 1, 1, 3, 3), cell::State::Dead);
        assert_eq!(cell::update(&nen, 1, 1, 3, 3), cell::State::Dead);
        assert_eq!(cell::update(&nwn, 1, 1, 3, 3), cell::State::Dead);
        assert_eq!(cell::update(&sen, 1, 1, 3, 3), cell::State::Dead);
        assert_eq!(cell::update(&swn, 1, 1, 3, 3), cell::State::Dead);
    }

    #[test]
    fn check_rule1_on_quad() {
        let a: [u8; 4] = color(cell::State::Alive).into();
        let d: [u8; 4] = color(cell::State::Dead).into();
        // all possible cases with 0 or 1 neighbour (<2)
        let alone = [d, d, d, d, a, d, d, d, d];
        let nn = [d, a, d, d, a, d, d, d, d];
        let sn = [d, d, d, d, a, d, d, a, d];
        let en = [d, d, d, d, a, a, d, d, d];
        let wn = [d, d, d, a, a, d, d, d, d];
        let nen = [d, d, a, d, a, d, d, d, d];
        let nwn = [a, d, d, d, a, d, d, d, d];
        let sen = [d, d, d, d, a, d, d, d, a];
        let swn = [d, d, d, d, a, d, a, d, d];
        assert_eq!(
            cell::update_on_quad(&alone, 1, 1, 3, 3),
            color(cell::State::Dead)
        );
        assert_eq!(
            cell::update_on_quad(&nn, 1, 1, 3, 3),
            color(cell::State::Dead)
        );
        assert_eq!(
            cell::update_on_quad(&sn, 1, 1, 3, 3),
            color(cell::State::Dead)
        );
        assert_eq!(
            cell::update_on_quad(&en, 1, 1, 3, 3),
            color(cell::State::Dead)
        );
        assert_eq!(
            cell::update_on_quad(&wn, 1, 1, 3, 3),
            color(cell::State::Dead)
        );
        assert_eq!(
            cell::update_on_quad(&nen, 1, 1, 3, 3),
            color(cell::State::Dead)
        );
        assert_eq!(
            cell::update_on_quad(&nwn, 1, 1, 3, 3),
            color(cell::State::Dead)
        );
        assert_eq!(
            cell::update_on_quad(&sen, 1, 1, 3, 3),
            color(cell::State::Dead)
        );
        assert_eq!(
            cell::update_on_quad(&swn, 1, 1, 3, 3),
            color(cell::State::Dead)
        );
    }

    #[test]
    fn check_rule3() {
        let a = cell::State::Alive;
        let d = cell::State::Dead;
        // all possible cases with 4, 5, 6, 7 or 8 neighbours (>3)
        let surrounded = vec![a, a, a, a, a, a, a, a, a];
        let seven: Vec<Vec<cell::State>> = vec![
            vec![d, a, a, a, a, a, a, a, a],
            vec![a, d, a, a, a, a, a, a, a],
            vec![a, a, d, a, a, a, a, a, a],
            vec![a, a, a, d, a, a, a, a, a],
            vec![a, a, a, a, a, d, a, a, a],
            vec![a, a, a, a, a, a, d, a, a],
            vec![a, a, a, a, a, a, a, d, a],
            vec![a, a, a, a, a, a, a, a, d],
        ];
        //TODO : combinations for 6, 5 and 4...

        assert_eq!(cell::update(&surrounded, 1, 1, 3, 3), cell::State::Dead);
        for s in seven {
            assert_eq!(cell::update(&s, 1, 1, 3, 3), cell::State::Dead);
        }
    }

    #[test]
    fn check_rule3_on_quad() {
        let a: [u8; 4] = color(cell::State::Alive).into();
        let d: [u8; 4] = color(cell::State::Dead).into();
        // all possible cases with 4, 5, 6, 7 or 8 neighbours (>3)
        let surrounded = [a, a, a, a, a, a, a, a, a];
        let seven = vec![
            [d, a, a, a, a, a, a, a, a],
            [a, d, a, a, a, a, a, a, a],
            [a, a, d, a, a, a, a, a, a],
            [a, a, a, d, a, a, a, a, a],
            [a, a, a, a, a, d, a, a, a],
            [a, a, a, a, a, a, d, a, a],
            [a, a, a, a, a, a, a, d, a],
            [a, a, a, a, a, a, a, a, d],
        ];
        //TODO : combinations for 6, 5 and 4...

        assert_eq!(
            cell::update_on_quad(&surrounded, 1, 1, 3, 3),
            color(cell::State::Dead)
        );
        for s in seven {
            assert_eq!(
                cell::update_on_quad(&s, 1, 1, 3, 3),
                color(cell::State::Dead)
            );
        }
    }

    #[test]
    fn check_rule2() {
        let a = cell::State::Alive;
        let d = cell::State::Dead;

        //TODO : all combinations for 2 and 3...
        let two = vec![d, a, a, d, a, d, d, d, d];
        let three = vec![d, d, a, d, a, a, a, d, d];

        assert_eq!(cell::update(&two, 1, 1, 3, 3), cell::State::Alive);
        assert_eq!(cell::update(&three, 1, 1, 3, 3), cell::State::Alive);
    }

    #[test]
    fn check_rule2_on_quad() {
        let a: [u8; 4] = color(cell::State::Alive).into();
        let d: [u8; 4] = color(cell::State::Dead).into();

        //TODO : all combinations for 2 and 3...
        let two = [d, a, a, d, a, d, d, d, d];
        let three = [d, d, a, d, a, a, a, d, d];

        assert_eq!(
            cell::update_on_quad(&two, 1, 1, 3, 3),
            color(cell::State::Alive)
        );
        assert_eq!(
            cell::update_on_quad(&three, 1, 1, 3, 3),
            color(cell::State::Alive)
        );
    }

    #[test]
    fn check_rule4() {
        let a = cell::State::Alive;
        let d = cell::State::Dead;

        let three = vec![d, d, a, d, d, a, a, d, d];

        assert_eq!(cell::update(&three, 1, 1, 3, 3), cell::State::Alive);
    }

    #[test]
    fn check_rule4_on_quad() {
        let a: [u8; 4] = color(cell::State::Alive).into();
        let d: [u8; 4] = color(cell::State::Dead).into();

        let three = [d, d, a, d, d, a, a, d, d];

        assert_eq!(
            cell::update_on_quad(&three, 1, 1, 3, 3),
            color(cell::State::Alive)
        );
    }

    #[test]
    fn dead_alone_check() {
        let a = cell::State::Alive;
        let d = cell::State::Dead;

        let two = vec![d, d, a, d, d, d, a, d, d];
        assert_eq!(cell::update(&two, 1, 1, 3, 3), cell::State::Dead);
    }

    #[test]
    fn dead_alone_check_on_quad() {
        let a: [u8; 4] = color(cell::State::Alive).into();
        let d: [u8; 4] = color(cell::State::Dead).into();

        let two = [d, d, a, d, d, d, a, d, d];
        assert_eq!(
            cell::update_on_quad(&two, 1, 1, 3, 3),
            color(cell::State::Dead)
        );
    }

    #[test]
    fn dead_in_crowd_check() {
        let a = cell::State::Alive;
        let d = cell::State::Dead;

        let four = vec![d, d, a, d, d, a, a, d, a];
        assert_eq!(cell::update(&four, 1, 1, 3, 3), cell::State::Dead);
    }

    #[test]
    fn dead_in_crowd_check_on_quad() {
        let a: [u8; 4] = ALIVE.into();
        let d: [u8; 4] = DEAD.into();

        let four = [d, d, a, d, d, a, a, d, a];
        assert_eq!(
            cell::update_on_quad(&four, 1, 1, 3, 3),
            color(cell::State::Dead)
        );
    }

    #[bench]
    fn bench_update(b: &mut Bencher) {
        let a = cell::State::Alive;
        let d = cell::State::Dead;

        let w = vec![d, d, a, d, d, a, a, d, a];

        b.iter(|| cell::update(&w, 1, 1, 3, 3));
    }

    #[bench]
    fn bench_update_on_quad(b: &mut Bencher) {
        let a: [u8; 4] = ALIVE.into();
        let d: [u8; 4] = DEAD.into();

        let w = [d, d, a, d, d, a, a, d, a];

        b.iter(|| cell::update_on_quad(&w, 1, 1, 3, 3));
    }
}
