use std::{collections::VecDeque, io, vec};

#[derive(Clone, Debug, PartialEq, Eq)]
enum Cell {
    Emp,
    Wall,
}

type Coord = (i32, i32);
type Field = Vec<Vec<Cell>>;

const DIRECTIONS: [Coord; 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];
const SIZE: i32 = 71;
const END: Coord = (SIZE - 1, SIZE - 1);
const START: Coord = (0, 0);

fn to_idx(pos: &Coord) -> (usize, usize) {
    (pos.0 as usize, pos.1 as usize)
}

fn move_dir(pos: &Coord, dir: &Coord, field: &Field) -> Option<Coord> {
    let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
    let idx = to_idx(&new_pos);
    if new_pos.0 < 0
        || new_pos.0 >= SIZE
        || new_pos.1 < 0
        || new_pos.1 >= SIZE
        || field[idx.0][idx.1] == Cell::Wall
    {
        None
    } else {
        Some(new_pos)
    }
}

fn bfs(field: &Field) -> bool {
    let mut queue: VecDeque<Coord> = VecDeque::new();
    queue.push_back(START);
    let mut visited = vec![vec![false; SIZE as usize]; SIZE as usize];
    visited[0][0] = true;

    while !queue.is_empty() {
        let pos = queue.pop_front().unwrap();
        if pos == END {
            return true;
        }

        let dirs = DIRECTIONS
            .iter()
            .map(|x| move_dir(&pos, x, field))
            .filter(|x| *x != None)
            .map(|x| x.unwrap());
        for dir in dirs {
            let idx = to_idx(&dir);
            if !visited[idx.0][idx.1] {
                queue.push_back(dir);
                visited[idx.0][idx.1] = true;
            }
        }
    }

    return false;
}

fn main() {
    let mut field = vec![vec![Cell::Emp; SIZE as usize]; SIZE as usize];
    let stdin = io::stdin();
    let mut buf = String::new();
    let mut positions: Vec<(usize, usize)> = Vec::new();

    loop {
        buf.clear();
        let res = stdin.read_line(&mut buf);
        if let Ok(0) = res {
            break;
        }
        let numbers = buf
            .trim()
            .split_terminator(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        positions.push((numbers[1], numbers[0]));
    }

    for i in 0..positions.len() {
        field[positions[i].0][positions[i].1] = Cell::Wall;
        if !bfs(&field) {
            println!("{:?}", positions[i]);
            break;
        }
    }
}
