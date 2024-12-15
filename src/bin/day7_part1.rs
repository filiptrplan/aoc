use std::io;

#[derive(Clone)]
enum Op {
    Plus,
    Mul,
}

struct Row {
    right: i64,
    numbers: Vec<i64>,
    ops: Vec<Op>,
}

fn eval(numbers: &Vec<i64>, ops: &Vec<Op>) -> i64 {
    numbers
        .iter()
        .map(|x| *x)
        .enumerate()
        .reduce(|(_, acc), (i, x)| match &ops[i - 1] {
            Op::Plus => (i, acc + x),
            Op::Mul => (i, acc * x),
        })
        .expect("No numbers")
        .1
}

fn check_valid(numbers: &Vec<i64>, ops: &mut Vec<Op>, pos: usize, right: i64) -> bool {
    if pos == ops.len() {
        return eval(numbers, ops) == right;
    }
    ops[pos] = Op::Plus;
    let res1 = check_valid(numbers, ops, pos + 1, right);
    ops[pos] = Op::Mul;
    let res2 = check_valid(numbers, ops, pos + 1, right);
    return res1 || res2;
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
        let numbers: Vec<i64> = first_split[1]
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
