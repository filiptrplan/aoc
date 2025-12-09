use std::io;

#[derive(PartialEq)]
enum Cell {
    Emp,
    Box,
    Wall,
}

type Coord = (i32, i32);

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn dir_to_pair(dir: Direction) -> (i32, i32) {
    match dir {
        Direction::Up => (-1, 0),
        Direction::Down => (1, 0),
        Direction::Left => (0, -1),
        Direction::Right => (0, 1),
    }
}

fn move_robot(robot_pos: &mut Coord, dir: Direction, field: &mut Vec<Vec<Cell>>) {
    let vector = dir_to_pair(dir);
    if field[(robot_pos.0 + vector.0) as usize][(robot_pos.1 + vector.1) as usize] == Cell::Emp {
        *robot_pos = (robot_pos.0 + vector.0, robot_pos.1 + vector.1);
    } else if field[(robot_pos.0 + vector.0) as usize][(robot_pos.1 + vector.1) as usize]
        == Cell::Box
    {
        let line_len = (1..1000)
            .find(|k| {
                field[(robot_pos.0 + vector.0 * k) as usize][(robot_pos.1 + vector.1 * k) as usize]
                    != Cell::Box
            })
            .unwrap() as i32;
        if field[(robot_pos.0 + line_len * vector.0) as usize]
            [(robot_pos.1 + line_len * vector.1) as usize]
            == Cell::Emp
        {
            field[(robot_pos.0 + line_len * vector.0) as usize]
                [(robot_pos.1 + line_len * vector.1) as usize] = Cell::Box;
            *robot_pos = (robot_pos.0 + vector.0, robot_pos.1 + vector.1);
            field[robot_pos.0 as usize][robot_pos.1 as usize] = Cell::Emp;
        }
    }
}

fn print_field(robot_pos: Coord, field: &Vec<Vec<Cell>>) {
    for i in 0..field.len() {
        for j in 0..field[i].len() {
            if (i as i32, j as i32) == robot_pos {
                print!("@");
            } else {
                match field[i][j] {
                    Cell::Wall => print!("#"),
                    Cell::Box => print!("O"),
                    Cell::Emp => print!("."),
                }
            }
        }
        println!("");
    }
}

fn main() {
    let stdin = io::stdin();
    let mut buf = String::new();
    let mut field: Vec<Vec<Cell>> = Vec::new();
    let mut robot_pos = (0, 0);
    let mut idx = (0, 0);
    let mut instructions: Vec<Direction> = Vec::new();

    loop {
        buf.clear();
        let _ = stdin.read_line(&mut buf);
        if buf == "\n" {
            break;
        }

        field.push(
            buf.trim()
                .as_bytes()
                .iter()
                .map(|x| {
                    let res = match x {
                        b'#' => Cell::Wall,
                        b'O' => Cell::Box,
                        b'.' => Cell::Emp,
                        b'@' => {
                            robot_pos = idx;
                            Cell::Emp
                        }
                        _ => Cell::Emp,
                    };
                    idx.1 += 1;
                    res
                })
                .collect(),
        );
        idx.0 += 1;
        idx.1 = 0;
    }

    loop {
        buf.clear();
        let res = stdin.read_line(&mut buf);
        if let Ok(0) = res {
            break;
        }

        instructions.append(
            &mut buf
                .trim_end()
                .as_bytes()
                .iter()
                .map(|x| match x {
                    b'v' => Direction::Down,
                    b'>' => Direction::Right,
                    b'<' => Direction::Left,
                    _ => Direction::Up,
                })
                .collect::<Vec<Direction>>(),
        );
    }

    for dir in instructions {
        move_robot(&mut robot_pos, dir, &mut field);
    }

    let res = field.iter().enumerate().map(|(i, row)|  {
            row.iter().enumerate().map(move |(j, cell)|  {
                if let Cell::Box = *cell {
                    100 * i + j
                } else {
                    0
                }
            }).sum::<usize>()
        }).sum::<usize>();
        println!("{}", res);
}
