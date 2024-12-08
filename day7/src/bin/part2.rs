use std::{cmp::min, io};

#[derive(Clone)]
enum Op {
    Plus,
    Mul,
    Con
}

struct Row {
    right: i128,
    numbers: Vec<i128>,
    ops: Vec<Op>,
}

fn concat(a: i128, b: i128, max:i128) -> i128 {
    let a_str = a.to_string();
    let b_str = b.to_string();
    (a_str + &b_str).parse().unwrap_or(max + 1)
}

fn eval(numbers: &Vec<i128>, ops: &Vec<Op>, right: i128) -> i128 {
    numbers
        .iter()
        .map(|x| *x)
        .enumerate()
        .reduce(|(_, acc), (i, x)| match &ops[i - 1] {
            Op::Plus => (i, acc + x),
            Op::Mul => (i, acc * x),
            Op::Con => (i, concat(acc, x, right))
        })
        .expect("No numbers")
        .1
}

fn check_valid(numbers: &Vec<i128>, ops: &mut Vec<Op>, pos: usize, right: i128) -> bool {
    if pos == ops.len() {
        return eval(numbers, ops, right) == right;
    }
    ops[pos] = Op::Plus;
    let res1 = check_valid(numbers, ops, pos + 1, right);
    ops[pos] = Op::Mul;
    let res2 = check_valid(numbers, ops, pos + 1, right);
    ops[pos] = Op::Con;
    let res3 = check_valid(numbers, ops, pos + 1, right);
    return res1 || res2 || res3;
}

pub fn main() {
    let stdin = io::stdin();
    let mut buf = String::new();
    let mut rows = Vec::new();
    loop {
        buf.clear();
        let res = stdin.read_line(&mut buf);
        if let Ok(0) = res {
            break;
        }
        let first_split: Vec<&str> = buf.split_terminator(":").collect();
        let numbers: Vec<i128> = first_split[1]
            .trim()
            .split_terminator(" ")
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| x.parse().expect("Failed to parse num"))
            .collect();
        let ops = vec![Op::Plus; numbers.len() - 1];
        rows.push(Row {
            numbers,
            ops,
            right: first_split[0].parse().expect("Failed to parse"),
        });
    }



    let res = rows
        .iter_mut()
        .map(|row| {
            (
                row.right,
                check_valid(&row.numbers, &mut row.ops, 0, row.right),
            )
        })
        .fold((0, false), |(acc, _), (x, check)| {
            if check {
                (acc + x, check)
            } else {
                (acc, check)
            }
        }).0;

    println!("{}", res);
}
