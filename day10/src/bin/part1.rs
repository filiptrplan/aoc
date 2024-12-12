use std::{collections::HashSet, io};

// y, x
type Coord = (i32, i32);

const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn bfs(field: &Vec<Vec<i32>>, visited: &mut HashSet<Coord>, curr_step: i32, coord: Coord) -> i32 {
    if coord.0 < 0
        || coord.1 < 0
        || coord.0 >= field.len() as i32
        || coord.1 >= field[0].len() as i32
    {
        return 0;
    }

    let u_coord = (coord.0 as usize, coord.1 as usize);

    if curr_step != field[u_coord.0][u_coord.1] {
        return 0;
    }

    if curr_step == 9 && field[u_coord.0][u_coord.1] == 9 {
        if visited.contains(&coord) {
            return 0;
        } else {
            visited.insert(coord);
            return 1;
        }
    } else if curr_step == 9 {
        return 0;
    }

    let mut res = 0;
    for dir in DIRECTIONS.iter() {
        let new_coord = (coord.0 + dir.0, coord.1 + dir.1);
        res += bfs(&field, visited, curr_step + 1, new_coord);
    }

    return res;
}

fn main() {
    let stdin = io::stdin();
    let mut buf = String::new();
    let mut field: Vec<Vec<i32>> = Vec::new();

    loop {
        buf.clear();
        let res = stdin.read_line(&mut buf);
        if let Ok(0) = res {
            break;
        }
        field.push(
            buf.trim_end()
                .as_bytes()
                .iter()
                .map(|x| (x - b'0') as i32)
                .collect(),
        );
    }

    let mut res = 0;
    for i in 0..field.len() {
        for j in 0..field[i].len() {
            if field[i][j] == 0 {
                let mut visited = HashSet::new();
                let score =  bfs(&field, &mut visited, 0, (i as i32, j as i32));
                res += score;
            }
        }
    }

    println!("{}", res);
}
