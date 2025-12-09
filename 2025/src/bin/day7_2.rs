use std::{collections::HashSet, env, fs, io, vec};

#[derive(Debug, PartialEq, Eq)]
enum Cell {
    Empty,
    Splitter,
    Beam,
}

/// (y,x)
type Coord = (usize, usize);

#[derive(Debug)]
struct Field {
    cells: Vec<Vec<Cell>>,
    start: Coord,
    memo: Vec<Vec<Option<u64>>>,
}

impl Field {
    fn new(input: &str) -> Self {
        let mut cells = Vec::new();
        let mut start = (0, 0);
        let lines = input.split("\n");
        for line in lines {
            cells.push(Vec::new());
            for c in line.chars() {
                let cell = match c {
                    'S' => {
                        start = (cells.len() - 1, cells.last().unwrap().len());
                        Cell::Empty
                    }
                    '.' => Cell::Empty,
                    '^' => Cell::Splitter,
                    '\n' => continue,
                    _ => panic!("invalid input"),
                };
                cells.last_mut().unwrap().push(cell);
            }
        }
        cells.retain(|row| !row.is_empty());
        let height = cells.len();
        let width = cells.last().unwrap().len();
        Self {
            cells,
            start,
            memo: vec![vec![None; width]; height],
        }
    }

    fn beam_count(&mut self) -> u64 {
        self.beam_count_from_start(self.start)
    }

    fn print(&self) {
        for row in self.cells.iter() {
            for cell in row {
                match cell {
                    Cell::Empty => print!("."),
                    Cell::Splitter => print!("^"),
                    Cell::Beam => print!("|"),
                }
            }
            println!();
        }
    }

    fn beam_count_from_start(&mut self, start: Coord) -> u64 {
        let start_x = start.1;
        let start_y = start.0;
        let splitter_y = self
            .cells
            .iter()
            .skip(start_y)
            .position(|row| row[start_x] == Cell::Splitter);

        match splitter_y {
            Some(pos) => {
                let global_splitter_y = start_y + pos;
                // if beam overlaps existing, return memoized
                if let Some(memoized) = self
                    .memo
                    .iter()
                    .skip(start_y)
                    .take(pos)
                    .find(|row| row[start_x].is_some())
                {
                    return memoized[start_x].unwrap();
                }

                let res = self.beam_count_from_start((global_splitter_y, start_x - 1))
                    + self.beam_count_from_start((global_splitter_y, start_x + 1));

                self.memo
                    .iter_mut()
                    .skip(start_y)
                    .take(pos)
                    .for_each(|row| row[start_x] = Some(res));
                res
            }
            // Fill the rest with beams
            None => {
                self.memo
                    .iter_mut()
                    .skip(start_y)
                    .for_each(|row| row[start_x] = Some(1));
                1
            }
        }
    }
}

fn main() {
    let path = env::args().next_back().unwrap();
    let input = fs::read_to_string(path).unwrap();
    let mut field = Field::new(&input);
    println!("{:?}", field.beam_count());
    // field.print();
}
