use std::{collections::HashSet, io};
use Square::*;

#[derive(Clone)]
enum Square {
    Obs,
    Emp,
}

type Pos = (i32, i32);
const UP: Pos = (0, -1);
const DOWN: Pos = (0, 1);
const RIGHT: Pos = (1, 0);
const LEFT: Pos = (-1, 0);

#[derive(Clone, Eq, Hash, PartialEq)]
struct Guard {
    pos: Pos,
    dir: Pos,
}

impl Guard {
    fn is_oob(&self, w: i32, h: i32) -> bool {
        self.pos.0 < 0 || self.pos.0 >= w || self.pos.1 < 0 || self.pos.1 >= h
    }

    fn turn(&mut self) {
        self.dir = match self.dir {
            UP => RIGHT,
            RIGHT => DOWN,
            DOWN => LEFT,
            LEFT => UP,
            _ => UP,
        };
    }

    fn forward(&mut self) {
        self.pos.0 += self.dir.0;
        self.pos.1 += self.dir.1;
    }
    fn faces_obs(&self, field: &Vec<Vec<Square>>, w: i32, h: i32) -> bool {
        let x = self.pos.0 + self.dir.0;
        let y = self.pos.1 + self.dir.1;
        if x < 0 || x >= w || y < 0 || y >= h {
            return false;
        }
        return match field[y as usize][x as usize] {
            Obs => true,
            _ => false,
        };
    }

    fn is_looped(&self, history: &mut HashSet<Guard>) -> bool {
        if !history.contains(self) {
            history.insert(self.clone());
            return false;
        } else {
            return true;
        }
    }
}

pub fn main() {
    let stdin = io::stdin();
    let mut buf = String::new();
    let mut field: Vec<Vec<Square>> = Vec::new();
    let mut start_pos = (0, 0);

    loop {
        buf.clear();
        let res = stdin.read_line(&mut buf);
        if let Ok(0) = res {
            break;
        }
        let mut row = Vec::new();
        for c in buf.as_bytes().into_iter() {
            match c {
                b'.' => row.push(Emp),
                b'#' => row.push(Obs),
                b'^' => {
                    row.push(Emp);
                    start_pos.0 = (row.len() - 1) as i32;
                    start_pos.1 = field.len() as i32;
                }
                _ => {}
            }
        }
        field.push(row);
    }

    let h = field.len() as i32;
    let w = field[0].len() as i32;

    let mut guard = Guard {
        pos: start_pos.clone(),
        dir: UP,
    };

    let mut obstacles = 0;
    let mut history: HashSet<Guard> = HashSet::new();
    // let i = 6;
    // let j = 3;
    for i in 0..h {
        println!("{}", i);
        for j in 0..w {
            history.clear();
            guard.pos = start_pos.clone();
            guard.dir = UP;
            if let Obs = field[i as usize][j as usize] {
                continue;
            }
            if (j, i) == start_pos {
                continue;
            }
            field[i as usize][j as usize] = Obs;
            // println!("Started {} {}", i, j);
            while !guard.is_oob(w, h) {
                while guard.faces_obs(&field, w, h) {
                    guard.turn();
                }
                if guard.is_looped(&mut history) {
                    // println!("Looped! {} {}", guard.pos.1, guard.pos.0);
                    obstacles += 1;
                    break;
                }
                guard.forward();
            }
            field[i as usize][j as usize] = Emp;
        }
    }

    println!("{}", obstacles);
}
