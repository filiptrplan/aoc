use std::{env, fs, vec};

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
    polygon: Polygon,
}

struct Polygon {
    coords: Vec<Coord>,
}

struct Edge {
    start: Coord,
    end: Coord,
}

impl Edge {
    fn is_vertical(&self) -> bool {
        self.start.1 == self.end.1 // same x
    }

    fn intersects(&self, other: &Edge) -> bool {
        if self.start == other.start
            || self.start == other.end
            || self.end == other.start
            || self.end == other.end
        {
            return false;
        }
        // vertical–vertical or horizontal–horizontal
        if self.is_vertical() == other.is_vertical() {
            if self.is_vertical() {
                // same x, overlapping y ranges
                if self.start.1 != other.start.1 {
                    return false;
                }
                let (a1, a2) = (self.start.0.min(self.end.0), self.start.0.max(self.end.0)); // y
                let (b1, b2) = (
                    other.start.0.min(other.end.0),
                    other.start.0.max(other.end.0),
                );
                return a1 <= b2 && b1 <= a2;
            } else {
                // same y, overlapping x ranges
                if self.start.0 != other.start.0 {
                    return false;
                }
                let (a1, a2) = (self.start.1.min(self.end.1), self.start.1.max(self.end.1)); // x
                let (b1, b2) = (
                    other.start.1.min(other.end.1),
                    other.start.1.max(other.end.1),
                );
                return a1 <= b2 && b1 <= a2;
            }
        }

        // one vertical, one horizontal
        let (v, h) = if self.is_vertical() {
            (self, other)
        } else {
            (other, self)
        };

        let vx = v.start.1;
        let vy1 = v.start.0.min(v.end.0);
        let vy2 = v.start.0.max(v.end.0);

        let hy = h.start.0;
        let hx1 = h.start.1.min(h.end.1);
        let hx2 = h.start.1.max(h.end.1);

        // (hy, vx) lies in both segments
        vx >= hx1 && vx <= hx2 && hy >= vy1 && hy <= vy2
    }
}

impl Polygon {
    fn new_from_corners(coord1: Coord, coord2: Coord) -> Self {
        let top = coord1.0.min(coord2.0);
        let bottom = coord1.0.max(coord2.0);
        let left = coord1.1.min(coord2.1);
        let right = coord1.1.max(coord2.1);

        let top_left = (top, left);
        let top_right = (top, right);
        let bottom_right = (bottom, right);
        let bottom_left = (bottom, left);
        Self {
            coords: vec![top_left, top_right, bottom_right, bottom_left],
        }
    }

    fn intersects_any_edge(&self, other: &Polygon) -> bool {
        self.edges()
            .any(|edge| other.edges().any(|edge2| edge.intersects(&edge2)))
    }

    fn edges(&self) -> impl Iterator<Item = Edge> {
        self.coords.iter().enumerate().map(|(i, coord)| {
            if i == self.coords.len() - 1 {
                Edge {
                    start: *coord,
                    end: self.coords[0],
                }
            } else {
                Edge {
                    start: *coord,
                    end: self.coords[i + 1],
                }
            }
        })
    }
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
        Self {
            coords: coords.clone(),
            polygon: Polygon {
                coords: coords.clone(),
            },
        }
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
                    .filter(|coord2| {
                        !self
                            .polygon
                            .intersects_any_edge(&Polygon::new_from_corners(*coord1, **coord2))
                    })
                    .map(|coord2| coord2.area(coord1));

                iter.max().unwrap_or(0)
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
