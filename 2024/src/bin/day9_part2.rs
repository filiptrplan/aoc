use core::num;
use std::{io, iter::repeat};

#[derive(Clone, Copy)]
enum Disk {
    Empty(u32),
    Id(u32, u32),
}

fn print(field: &Vec<Disk>) {
    field.iter().for_each(|x| match x {
        Disk::Empty(len) => print!("{}", repeat(".").take(*len as usize).collect::<String>()),
        Disk::Id(i, len) => print!(
            "{}",
            repeat(i.to_string())
                .take(*len as usize)
                .collect::<String>()
        ),
    });
    println!("");
}

fn next_occ(field: &Vec<Disk>, prev_id: Option<u32>) -> usize {
    let mut idx = field.len() - 1;
    let less_id = match prev_id {
        None => 1000000,
        Some(i) => i,
    };
    while idx >= 0 {
        if let Disk::Id(id, _) = field[idx] {
            if id < less_id {
                return idx;
            }
        }
        idx -= 1;
    }
    idx
}

fn main() {
    let mut field: Vec<Disk> = Vec::new();
    let stdin = io::stdin();
    let mut buf = String::new();

    let _ = stdin.read_line(&mut buf);
    let mut id: u32 = 0;
    let mut free_space = false;
    for c in buf.trim_end().as_bytes().iter() {
        let number = c - b'0';
        if free_space {
            field.push(Disk::Empty(number.into()));
        } else {
            field.push(Disk::Id(id.into(), number.into()));
            id += 1;
        }
        free_space = !free_space;
    }

    let mut prev_num = None;

    loop {
        let mut occ_idx = next_occ(&field, prev_num);
        if occ_idx == 0 || prev_num.unwrap_or(1) == 0 {
            break;
        }

        prev_num = match field[occ_idx] {
            Disk::Id(id, _) => Some(id),
            Disk::Empty(_) => None,
        };

        let num_len = match field[occ_idx] {
            Disk::Empty(_) => None,
            Disk::Id(_, len) => Some(len),
        };

        let mut free_idx = 0;
        let mut emp_len = 0;
        while free_idx < occ_idx {
            if let Disk::Empty(len_empty) = field[free_idx] {
                if len_empty >= num_len.expect("Should exist") {
                    emp_len = len_empty;
                    break;
                }
            }
            free_idx += 1;
        }

        if free_idx < occ_idx {
            field[free_idx] = field[occ_idx];
            field[occ_idx] = Disk::Empty(num_len.expect("Should exist"));

            let mut curr_len = num_len.unwrap();

            // Check the 2 neighbours and merge the empty ones
        // print(&field);
            if free_idx > 0 {
                if let Disk::Empty(left_len) = field[occ_idx - 1] {
                    curr_len = left_len + curr_len;
                    field[occ_idx - 1] = Disk::Empty(curr_len);
                    field.remove(occ_idx);
                    occ_idx -= 1;
                }
            }

            if occ_idx < field.len() - 1 {
                if let Disk::Empty(right_len) = field[occ_idx + 1] {
                    curr_len = right_len + curr_len;
                    field[occ_idx] = Disk::Empty(curr_len);
                    field.remove(occ_idx + 1);
                }
            }

            if emp_len != num_len.unwrap() {
                field.insert(
                    free_idx + 1,
                    Disk::Empty(emp_len - num_len.expect("Should exist")),
                );
            }
        }
        // print(&field);
    }

    let mut result: u128 = 0;
    let mut idx = 0;
    for i in 0..field.len() {
        match field[i] {
            Disk::Empty(len) => {
                idx += len;
            }
            Disk::Id(id, len) => {
                result += (idx..idx + len).fold(0, |acc, b| acc + (id * b)) as u128;
                idx += len;
            }
        }
    }

    println!("{}", result);
}
