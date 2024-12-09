use std::{io, mem::swap};

#[derive(Clone, Copy)]
enum Disk {
    Empty,
    Id(u32),
}

fn print(field: &Vec<Disk>) {
    field.iter().for_each(|x| match x {
        Disk::Empty => print!("."),
        Disk::Id(i) => print!("{}", i),
    });
    println!("");
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
            field.append(&mut vec![Disk::Empty; number.into()]);
        } else {
            field.append(&mut vec![Disk::Id(id); number.into()]);
            id += 1;
        }
        free_space = !free_space;
    }

    let mut free_idx = 0;
    let mut occ_idx = field.len() - 1;

    // print(&field);
    loop {
        while free_idx < field.len() {
            if let Disk::Empty = field[free_idx] {
                break;
            }
            free_idx += 1;
        }
        while occ_idx > 0 {
            if let Disk::Id(_) = field[occ_idx] {
                break;
            }
            occ_idx -= 1;
        }
        if free_idx >= field.len() || occ_idx >= field.len() || free_idx >= occ_idx{
            break;
        }

        field[free_idx] = field[occ_idx];
        field[occ_idx] = Disk::Empty;
        // print(&field);
    }

    let mut result: u128 = 0;
    for i in 0..field.len() {
        if let Disk::Id(id) = field[i] {
            result += (i * id as usize) as u128;
        }
    }

    println!("{}", result);
}
