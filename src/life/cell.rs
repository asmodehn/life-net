use crate::life::usize_from_i32;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum State {
    Alive,
    Dead,
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

pub fn update(cells: &Vec<State>, x: i32, y: i32, w: usize, h: usize) -> State {
    let neighbors_count = neighbours_count(&cells, x, y, w, h);

    let current_cell = cells[usize_from_i32(y * w as i32 + x)];

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
    fn check_rule4() {
        let a = cell::State::Alive;
        let d = cell::State::Dead;

        let three = vec![d, d, a, d, d, a, a, d, d];

        assert_eq!(cell::update(&three, 1, 1, 3, 3), cell::State::Alive);
    }

    #[test]
    fn dead_alone_check() {
        let a = cell::State::Alive;
        let d = cell::State::Dead;

        let two = vec![d, d, a, d, d, d, a, d, d];
        assert_eq!(cell::update(&two, 1, 1, 3, 3), cell::State::Dead);
    }

    #[test]
    fn dead_in_crowd_check() {
        let a = cell::State::Alive;
        let d = cell::State::Dead;

        let four = vec![d, d, a, d, d, a, a, d, a];
        assert_eq!(cell::update(&four, 1, 1, 3, 3), cell::State::Dead);
    }
}
