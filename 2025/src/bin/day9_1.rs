use std::{env, fs};

use aoc_2025::Coord;

trait Area {
    fn area(&self, other: &Coord) -> u64;
}

impl Area for Coord {
    fn area(&self, other: &Coord) -> u64 {
        ((other.0.max(self.0) - other.0.min(self.0) + 1)
            * (other.1.max(self.1) - other.1.min(self.1) + 1))
            .try_into()
            .expect("Failed to tryinto")
    }
}

struct Field {
    coords: Vec<Coord>,
}

impl Field {
    pub fn new(input: &str) -> Self {
        let lines = input.split("\n");
        let mut coords = Vec::new();
        for line in lines {
            let line_coord = line.split(",").collect::<Vec<_>>();
            if line_coord.len() < 2 {
                continue;
            }
            coords.push((
                line_coord[0].parse().unwrap(),
                line_coord[1].parse().unwrap(),
            ));
        }
        Self { coords }
    }

    fn max_area(&self) -> u64 {
        self.coords
            .iter()
            .enumerate()
            .map(|(i, coord1)| {
                let iter = self
                    .coords
                    .iter()
                    .skip(i + 1)
                    .map(|coord2| coord2.area(coord1));

                if iter.len() == 0 {
                    return 0;
                }
                iter.max().unwrap()
            })
            .max()
            .unwrap()
    }
}

fn main() {
    let path = env::args().next_back().unwrap();
    let input = fs::read_to_string(path).unwrap();
    let field = Field::new(&input);
    println!("{}", field.max_area());
}
