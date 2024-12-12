use std::io;

use disjoint::DisjointSet;

type Coord = (usize, usize);

struct Region {
    label: usize,
    plant: u8,
    coordinates: Vec<Coord>,
}

impl Region {
    fn gen_coords(&mut self, field_labels: &Vec<Vec<usize>>) {
        for i in 0..field_labels.len() {
            for j in 0..field_labels[i].len() {
                if field_labels[i][j] == self.label {
                    self.coordinates.push((i, j));
                }
            }
        }
    }

    fn area(&self) -> u32 {
        self.coordinates.len() as u32
    }

    fn perimeter(&self, field_labels: &Vec<Vec<usize>>) -> u32 {
        (0..self.coordinates.len())
            .map(|i| self.cell_corners(i, field_labels))
            .fold(0, |a, b| a + b)
    }

    fn cell_corners(&self, idx: usize, field_labels: &Vec<Vec<usize>>) -> u32 {
        let coord = self.coordinates[idx];
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut corners = 0;

        for i in 0..directions.len() {
            let c1 = directions[i];
            let c2 = directions[(i + 1) % directions.len()];
            let new_x = coord.0 as isize + c1.0;
            let new_y = coord.1 as isize + c1.1;
            let new_x2 = coord.0 as isize + c2.0;
            let new_y2 = coord.1 as isize + c2.1;

            if (new_x < 0
                || new_y < 0
                || new_x >= field_labels.len() as isize
                || new_y >= field_labels[0].len() as isize
                || field_labels[new_x as usize][new_y as usize] != self.label)
                && (new_x2 < 0
                    || new_y2 < 0
                    || new_x2 >= field_labels.len() as isize
                    || new_y2 >= field_labels[0].len() as isize
                    || field_labels[new_x2 as usize][new_y2 as usize] != self.label)
            {
                corners += 1;
            } else if (new_x >= 0
                && new_y >= 0
                && new_x < field_labels.len() as isize
                && new_y < field_labels[0].len() as isize
                && field_labels[new_x as usize][new_y as usize] == self.label)
                && (new_x2 >= 0
                    && new_y2 >= 0
                    && new_x2 < field_labels.len() as isize
                    && new_y2 < field_labels[0].len() as isize
                    && field_labels[new_x2 as usize][new_y2 as usize] == self.label)
                && !(new_x + c2.0 >= 0
                    && new_y + c2.1 >= 0
                    && new_x + c2.0 < field_labels.len() as isize
                    && new_y + c2.1 < field_labels[0].len() as isize
                    && field_labels[(new_x + c2.0) as usize][(new_y + c2.1) as usize] == self.label)
            {
                corners += 1;
            }
        }
        corners
    }
}

fn main() {
    let stdin = io::stdin();
    let mut buf = String::new();
    let mut field: Vec<Vec<u8>> = Vec::new();
    let mut field_regions: Vec<Vec<usize>> = Vec::new();
    let mut regions: Vec<Region> = Vec::new();
    let mut equivalences: Vec<(usize, usize)> = Vec::new();

    loop {
        buf.clear();
        let res = stdin.read_line(&mut buf);
        if let Ok(0) = res {
            break;
        }

        field.push(buf.trim_end().as_bytes().to_vec());
    }

    for i in 0..field.len() {
        field_regions.push(Vec::new());
        for j in 0..field[i].len() {
            let (region_up, field_up) = if (i as isize) - 1 >= 0 {
                (Some(field_regions[i - 1][j]), Some(field[i - 1][j]))
            } else {
                (None, None)
            };
            let (region_left, field_left) = if (j as isize) - 1 >= 0 {
                (Some(field_regions[i][j - 1]), Some(field[i][j - 1]))
            } else {
                (None, None)
            };
            match (field_up, field_left) {
                (Some(f), None) => {
                    if f == field[i][j] {
                        field_regions[i].push(region_up.unwrap());
                        continue;
                    }
                }
                (None, Some(f)) => {
                    if f == field[i][j] {
                        field_regions[i].push(region_left.unwrap());
                        continue;
                    }
                }
                (Some(f1), Some(f2)) => {
                    if f1 == field[i][j] && f2 == field[i][j] {
                        field_regions[i].push(region_left.unwrap());
                        equivalences.push((region_left.unwrap(), region_up.unwrap()));
                        continue;
                    } else if f1 == field[i][j] {
                        field_regions[i].push(region_up.unwrap());
                        continue;
                    } else if f2 == field[i][j] {
                        field_regions[i].push(region_left.unwrap());
                        continue;
                    }
                }
                (None, None) => {}
            }
            regions.push(Region {
                label: regions.len(),
                coordinates: Vec::new(),
                plant: field[i][j],
            });
            field_regions[i].push(regions.len() - 1);
        }
    }

    let mut labels = DisjointSet::with_len(regions.len());
    for eq in equivalences.iter() {
        labels.join(eq.0, eq.1);
    }

    for i in 0..field.len() {
        for j in 0..field[i].len() {
            field_regions[i][j] = labels.root_of(field_regions[i][j]);
        }
    }

    regions
        .iter_mut()
        .for_each(|r| r.gen_coords(&field_regions));

    // println!("{:?}", equivalences);
    // field_regions.iter().for_each(|r| println!("{:?}", r));

    let res1 = regions
        .iter()
        .map(|x| (x.plant as char, x.area(), x.perimeter(&field_regions)))
        .collect::<Vec<(char, u32, u32)>>();

    println!("{:?}", res1);
    // .fold(0, |acc, (a, b)| acc + a * b);

    let res = regions
        .iter()
        .map(|x| (x.area(), x.perimeter(&field_regions)))
        .fold(0, |acc, (a, b)| acc + a * b);

    println!("{:?}", res);
}
