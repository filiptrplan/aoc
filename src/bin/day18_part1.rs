use std::{collections::VecDeque, io, vec};

#[derive(Clone, Debug, PartialEq, Eq)]
enum Cell {
    Emp,
    Wall,
}

type Coord = (i32, i32);
type Field = Vec<Vec<Cell>>;

struct Search {
    pos: Coord,
    cost: i32,
}

const DIRECTIONS: [Coord; 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];
const SIZE: i32 = 71;
const END: Coord = (SIZE - 1, SIZE - 1);
const START: Coord = (0, 0);
const READ_BYTES: i32 = 1024;

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

fn bfs(field: &Field) -> i32 {
    let mut queue: VecDeque<Search> = VecDeque::new();
    queue.push_back(Search {
        pos: START,
        cost: 0,
    });
    let mut visited = vec![vec![false; SIZE as usize]; SIZE as usize];
    visited[0][0] = true;

    while !queue.is_empty() {
        let search = queue.pop_front().unwrap();
        if search.pos == END {
            return search.cost;
        }

        let dirs = DIRECTIONS
            .iter()
            .map(|x| move_dir(&search.pos, x, field))
            .filter(|x| *x != None)
            .map(|x| x.unwrap());
        for dir in dirs {
            let idx = to_idx(&dir);
            if !visited[idx.0][idx.1] {
                queue.push_back(Search {
                    pos: dir,
                    cost: search.cost + 1,
                });
                visited[idx.0][idx.1] = true;
            }
        }
    }

    return 0;
}

fn main() {
    let mut field = vec![vec![Cell::Emp; SIZE as usize]; SIZE as usize];
    let stdin = io::stdin();
    let mut buf = String::new();
    let mut i = 0;

    while i < READ_BYTES {
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
        field[numbers[1]][numbers[0]] = Cell::Wall;
        i += 1;
    }

    println!("{}", bfs(&field));
}
