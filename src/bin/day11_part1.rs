use std::io;

fn check_even(a: u128) -> bool {
    a.to_string().len() % 2 == 0
}

fn split_number(a: u128) -> (u128, u128) {
    let a_str = a.to_string();
    let halves = a_str.split_at(a_str.len() / 2);
    (halves.0.parse().unwrap(), halves.1.parse().unwrap())
}

fn mutate(stones: Vec<u128>) -> Vec<u128> {
    let mut new_stones = Vec::new();

    for stone in stones {
        if stone == 0 {
            new_stones.push(1);
        } else if check_even(stone) {
            let halves = split_number(stone);
            new_stones.push(halves.0);
            new_stones.push(halves.1);
        } else {
            new_stones.push(2024 * stone);
        }
    }

    new_stones
}

fn main() {
    let stdin = io::stdin();
    let mut buf = String::new();

    let _ = stdin.read_line(&mut buf);
    let mut stones: Vec<u128> = buf
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    for _ in 0..25 {
        stones = mutate(stones);
    }

    println!("{}", stones.len());
}
