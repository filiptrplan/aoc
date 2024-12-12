use std::{collections::HashMap, io};

fn check_even(a: u128) -> bool {
    a.to_string().len() % 2 == 0
}

fn split_number(a: u128) -> (u128, u128) {
    let a_str = a.to_string();
    let halves = a_str.split_at(a_str.len() / 2);
    (halves.0.parse().unwrap(), halves.1.parse().unwrap())
}

fn mutate(stone: u128, levels_left: u32, memo: &mut HashMap<(u128, u32), u128>) -> u128 {
    if memo.contains_key(&(stone, levels_left)) {
        return memo.get(&(stone, levels_left)).unwrap().clone();
    }
    if levels_left == 0 {
        return 1;
    }
    let res;
    if stone == 0 {
        res = mutate(1, levels_left - 1, memo);
    } else if check_even(stone) {
        let halves = split_number(stone);
        res = mutate(halves.0, levels_left - 1, memo) + mutate(halves.1, levels_left - 1, memo);
    } else {
        res = mutate(stone * 2024, levels_left - 1, memo);
    }
    memo.insert((stone, levels_left), res);
    return res;
}

fn main() {
    let stdin = io::stdin();
    let mut buf = String::new();

    let _ = stdin.read_line(&mut buf);
    let stones: Vec<u128> = buf
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();


    let mut memo = HashMap::new();

    let res = stones.iter().map(|x| mutate(*x, 75, &mut memo)).fold(0, |a,b| a + b);

    println!("{}", res);
}
