use std::io;
const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;
const SECONDS: i32 = 100;
// y, x
type Coord = (i32, i32);

#[derive(Debug)]
struct Robot {
    init_pos: Coord,
    vel: Coord,
}

fn eq(a: i32, base: i32) -> i32 {
    let mut i = a;
    while i < 0 {
        i += base;
    }
    i % base
}

fn check_line(robots: &Vec<Robot>) -> bool {
    robots.iter().any(|robot| {
        // Check for line 16 long
        (0..16).all(|k| {
            robots
                .iter()
                .any(|robot2| robot2.init_pos == (robot.init_pos.0 + k, robot.init_pos.1))
        })
    })
}

fn main() {
    let stdin = io::stdin();
    let mut buf = String::new();
    let mut robots = Vec::new();

    let re = regex::Regex::new(r"p=(-?[0-9]+),(-?[0-9]+) v=(-?[0-9]+),(-?[0-9]+)").unwrap();

    loop {
        buf.clear();
        let res = stdin.read_line(&mut buf);
        if let Ok(0) = res {
            break;
        }
        let captures = re.captures(&buf).unwrap();
        robots.push(Robot {
            init_pos: (captures[1].parse().unwrap(), captures[2].parse().unwrap()),
            vel: (captures[3].parse().unwrap(), captures[4].parse().unwrap()),
        });
    }

    for k in 1..1000000 {
        robots.iter_mut().for_each(|robot| {
            robot.init_pos = (
                eq(robot.init_pos.0 + robot.vel.0, WIDTH),
                eq(robot.init_pos.1 + robot.vel.1, HEIGHT),
            )
        });
        if check_line(&robots) {
            println!("{}", k);
            let pos = robots.iter().map(|x| x.init_pos).collect::<Vec<Coord>>();
            for i in 0..HEIGHT {
                for j in 0..WIDTH {
                    if pos.contains(&(j as i32, i as i32)) {
                        print!("X")
                    } else {
                        print!(".")
                    }
                }
                println!("")
            }
        }
    }

    // final_pos.clone().for_each(|x| println!("{:?}", x));
    // let res = final_pos
    //     .iter()
    //     .map(|pos| match *pos {
    //         (x, y) if y < HEIGHT / 2 && x < WIDTH / 2 => 1,
    //         (x, y) if y < HEIGHT / 2 && x >= WIDTH / 2 + 1 => 2,
    //         (x, y) if y >= HEIGHT / 2 + 1 && x < WIDTH / 2 => 3,
    //         (x, y) if y >= HEIGHT / 2 + 1 && x >= WIDTH / 2 + 1 => 4,
    //         _ => 0,
    //     })
    //     .fold([0, 0, 0, 0, 0], |mut arr, x| {
    //         arr[x] += 1;
    //         arr
    //     });
    // println!("{:?}, {} {}", res, WIDTH / 2, HEIGHT / 2);
    // println!("{}", res.iter().rev().take(4).fold(1, |a, b| a * b))
}
