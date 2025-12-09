use std::io;
use Square::*;

#[derive(Clone)]
enum Square {
    Obs,
    Emp,
    Vis,
}

type Pos = (i32, i32);
const UP: Pos = (0, -1);
const DOWN: Pos = (0, 1);
const RIGHT: Pos = (1, 0);
const LEFT: Pos = (-1, 0);

#[derive(Clone)]
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
    fn faces_obs(&self, field: &Vec<Vec<Square>>) -> bool {
        let h = field.len() as i32;
        let w = field[0].len() as i32;
        let mut moved = self.clone();
        moved.forward();
        if !moved.is_oob(w, h) {
            return match field[moved.pos.1 as usize][moved.pos.0 as usize] {
                Obs => true,
                _ => false,
            };
        }
        return false;
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
        pos: start_pos,
        dir: UP,
    };

    let mut visited = 0;

    while !guard.is_oob(w, h) {
        let cur = field[guard.pos.1 as usize][guard.pos.0 as usize].clone();
        if let Emp = cur  {
            visited += 1;
            field[guard.pos.1 as usize][guard.pos.0 as usize] = Vis;
        }
        while guard.faces_obs(&field) {
            guard.turn();
        }
        guard.forward();
    }

    println!("{}", visited);
}
