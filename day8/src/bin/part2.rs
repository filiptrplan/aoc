use std::{
    collections::{HashMap, HashSet},
    io,
};

// y, x
type Pos = (usize, usize);

struct ChooseIter {
    max: usize,
    i: usize,
    j: usize,
}

impl ChooseIter {
    fn new(max: usize) -> ChooseIter {
        ChooseIter {
            max: max - 1,
            i: 0,
            j: 0,
        }
    }
}

impl Iterator for ChooseIter {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.i;
        let j = self.j;
        let max = self.max;
        if (0, 0) == (i, j) {
            self.j = 1;
            Some((0, 1))
        } else if j < max {
            self.j += 1;
            Some((self.i, self.j))
        } else if j == max && i < max - 1 {
            self.i += 1;
            self.j = self.i + 1;
            Some((self.i, self.j))
        } else {
            None
        }
    }
}

fn antinode_pos(pos1: &Pos, pos2: &Pos) -> ((i32, i32), (i32, i32)) {
    // Dodam enici
    let pos1i = (pos1.0 as i32, pos1.1 as i32);
    let pos2i = (pos2.0 as i32, pos2.1 as i32);
    let diff1 = (pos1i.0 - pos2i.0, pos1i.1 - pos2i.1);
    let diff2 = (pos2i.0 - pos1i.0, pos2i.1 - pos1i.1);
    (
        (pos1i.0 + diff1.0, pos1i.1 + diff1.1),
        (pos2i.0 + diff2.0, pos2i.1 + diff2.1),
    )
}

fn gen_antenna_positions(field: &Vec<Vec<u8>>) -> HashMap<u8, Vec<Pos>> {
    let mut positions = HashMap::new();
    for (i, row) in field.iter().enumerate() {
        for (j, ch) in row.iter().enumerate() {
            if ch.is_ascii_alphanumeric() {
                if !positions.contains_key(ch) {
                    positions.insert(ch.clone(), Vec::new());
                }
                if let Some(pos) = positions.get_mut(ch) {
                    pos.push((i, j));
                }
            }
        }
    }
    positions
}

fn gen_antinode_pos(positions: &HashMap<u8, Vec<Pos>>) -> HashSet<(i32, i32)> {
    let mut set: HashSet<(i32, i32)> = HashSet::new();
    for (_, pos_vec) in positions {
        let choose = ChooseIter::new(pos_vec.len());
        for (p1, p2) in choose {
            let (a1, a2) = antinode_pos(&pos_vec[p1], &pos_vec[p2]);
            set.insert(a1);
            set.insert(a2);
        }
    }
    set
}

fn main() {
    let stdin = io::stdin();
    let mut buf = String::new();
    let mut field: Vec<Vec<u8>> = Vec::new();

    loop {
        buf.clear();
        let res = stdin.read_line(&mut buf);
        if let Ok(0) = res {
            break;
        }
        field.push(buf.trim().as_bytes().to_vec());
    }
    let h = field.len() as i32;
    let w = field[0].len() as i32;
    let ant_pos = gen_antenna_positions(&field);
    let set_positions = gen_antinode_pos(&ant_pos);
    let filtered_positions = set_positions
        .into_iter()
        .filter(|x| !(x.0 < 0 || x.1 < 0 || x.0 >= h || x.1 >= w))
        .collect::<Vec<(i32, i32)>>();

    println!("{}", filtered_positions.len());
}
