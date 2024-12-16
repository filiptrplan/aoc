use std::{collections::BinaryHeap, io};

type Coord = (usize, usize);

#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Eq, Debug)]
struct State {
    position: Coord,
    dir: Dir,
    cost: u128,
    path_cost: u128,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.path_cost.cmp(&other.path_cost))
            .then_with(|| self.position.cmp(&other.position))
            .then_with(|| self.dir.cmp(&other.dir))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Cell {
    Wall,
    Emp,
}

struct Problem {
    queue: BinaryHeap<State>,
    field: Vec<Vec<Cell>>,
    field_cost: Vec<Vec<Vec<u128>>>,
    start: Coord,
    end: Coord,
}

#[derive(Eq, PartialEq)]
enum Quadrant {
    TopR,
    BotR,
    BotL,
    TopL,
    Right,
    Left,
    Down,
    Up,
}

impl Dir {
    fn diff(&self, other: &Dir) -> u128 {
        use Dir::*;
        match (self, other) {
            (a, b) if a == b => 0,
            (Up, Right) => 1,
            (Up, Left) => 1,
            (Up, Down) => 2,
            (Right, Down) => 1,
            (Right, Left) => 2,
            (Down, Left) => 1,
            _ => other.diff(&self),
        }
    }
    fn idx(&self) -> usize {
        match self {
            Dir::Down => 0,
            Dir::Right => 1,
            Dir::Up => 2,
            Dir::Left => 3,
        }
    }
}

impl Problem {
    fn new(buf: String) -> Problem {
        let split_lines: Vec<&str> = buf.split_terminator("\n").collect();
        let w = split_lines[0].len();
        let h = split_lines.len();
        let s_idx = buf.replace("\n", "").find("S").unwrap();
        let e_idx = buf.replace("\n", "").find("E").unwrap();
        Problem {
            queue: BinaryHeap::new(),
            field: split_lines
                .iter()
                .map(|row| {
                    row.as_bytes()
                        .iter()
                        .map(|c| match c {
                            b'#' => Cell::Wall,
                            _ => Cell::Emp,
                        })
                        .collect::<Vec<Cell>>()
                })
                .collect(),
            start: (s_idx / w, s_idx % w),
            end: (e_idx / w, e_idx % w),
            field_cost: vec![vec![vec![u128::MAX; 4]; w]; h],
        }
    }

    fn heuristic(&self, pos: Coord, dir: Dir) -> u128 {
        if pos == self.end {
            return 0;
        }
        let manhattan =
            (pos.0 as i128 - self.end.0 as i128).abs() + (pos.1 as i128 - self.end.1 as i128).abs();
        let quadrant = if pos.0 == self.end.0 {
            // Same height
            if pos.1 < self.end.1 {
                Quadrant::Right
            } else {
                Quadrant::Left
            }
        } else if pos.0 < self.end.0 {
            // Above
            if pos.1 < self.end.1 {
                Quadrant::BotR
            } else if pos.1 > self.end.1 {
                Quadrant::BotL
            } else {
                Quadrant::Down
            }
        } else {
            // Below
            if pos.1 < self.end.1 {
                Quadrant::TopR
            } else if pos.1 > self.end.1 {
                Quadrant::TopL
            } else {
                Quadrant::Up
            }
        };
        use Quadrant::*;
        let turns = match quadrant {
            Up => dir.diff(&Dir::Up),
            Down => dir.diff(&Dir::Down),
            Left => dir.diff(&Dir::Left),
            Right => dir.diff(&Dir::Right),
            TopR => {
                if dir == Dir::Up || dir == Dir::Right {
                    1
                } else {
                    2
                }
            }
            BotR => {
                if dir == Dir::Down || dir == Dir::Right {
                    1
                } else {
                    2
                }
            }
            TopL => {
                if dir == Dir::Up || dir == Dir::Left {
                    1
                } else {
                    2
                }
            }
            BotL => {
                if dir == Dir::Down || dir == Dir::Left {
                    1
                } else {
                    2
                }
            }
        };
        turns * 1000 + manhattan as u128
    }

    fn move_state(&self, state: &State) -> Option<Coord> {
        let move_vec = match state.dir {
            Dir::Down => (1, 0),
            Dir::Up => (-1, 0),
            Dir::Left => (0, -1),
            Dir::Right => (0, 1),
        };
        let new_pos = (
            state.position.0 as i32 + move_vec.0,
            state.position.1 as i32 + move_vec.1,
        );
        if new_pos.0 < 0
            || new_pos.1 < 0
            || new_pos.0 >= self.field.len() as i32
            || new_pos.1 >= self.field[0].len() as i32
        {
            None
        } else {
            let new_pos_u = (new_pos.0 as usize, new_pos.1 as usize);
            if self.field[new_pos_u.0][new_pos_u.1] == Cell::Wall {
                None
            } else {
                Some(new_pos_u)
            }
        }
    }

    fn solve(&mut self) -> Option<u128> {
        self.queue.push(State {
            cost: self.heuristic(self.start, Dir::Right),
            path_cost: 0,
            position: self.start,
            dir: Dir::Right,
        });

        while !self.queue.is_empty() {
            // Get the best state
            let state = self.queue.pop().unwrap();
            // println!("{:?}", state);
            if state.position == self.end {
                return Some(state.path_cost);
            }
            self.field_cost[state.position.0][state.position.1][state.dir.idx()] = state.path_cost;
            // Generate its neighbours
            // First generate the turns
            let other_turns = [Dir::Up, Dir::Left, Dir::Down, Dir::Right]
                .into_iter()
                .filter(|x| x.diff(&state.dir) != 0);
            for turn in other_turns {
                let new_cost = state.path_cost + 1000 * turn.diff(&state.dir);
                if new_cost < self.field_cost[state.position.0][state.position.1][turn.idx()] {
                    self.queue.push(State {
                        position: state.position,
                        path_cost: new_cost,
                        cost: new_cost + self.heuristic(state.position, turn),
                        dir: turn,
                    });
                }
            }
            // Then the forward move
            if let Some(new_pos) = self.move_state(&state) {
                let new_cost = state.path_cost + 1;
                if new_cost < self.field_cost[new_pos.0][new_pos.1][state.dir.idx()] {
                    self.queue.push(State {
                        position: new_pos,
                        path_cost: new_cost,
                        cost: new_cost + self.heuristic(new_pos, state.dir),
                        dir: state.dir,
                    });
                }
            }
        };
        None
    }
}

fn main() {
    let stdin = io::stdin();
    let mut buf = String::new();
    loop {
        let res = stdin.read_line(&mut buf);
        if let Ok(0) = res {
            break;
        }
    }

    let mut pr = Problem::new(buf);
    println!("{:?}", pr.solve());
}
