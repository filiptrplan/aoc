use crate::Direction::*;
use std::io;

#[derive(PartialEq)]
enum Direction {
    Inc,
    Dec,
    Neut,
    Unk,
}

fn parse_report(report: Vec<i32>) -> bool {
    report
        .into_iter()
        .map(|x| (x, true, Direction::Unk))
        .reduce(|(xprev, res, dir_prev), (x, _, _)| {
            let dir = match dir_prev {
                Direction::Unk => {
                    if x - xprev > 0 {
                        Inc
                    } else {
                        Dec
                    }
                }
                _ => dir_prev,
            };
            let curr_dir = if x - xprev > 0 {
                Direction::Inc
            } else if x - xprev < 0 {
                Direction::Dec
            } else {
                Direction::Neut
            };
            if curr_dir != dir {
                (x, false, dir)
            } else {
                (
                    x,
                    res && (1 <= (x - xprev).abs() && (x - xprev).abs() <= 3),
                    dir,
                )
            }
        })
        .expect("Failed to iter")
        .1
}

fn main() {
    let mut inp = String::new();
    let stdin = io::stdin();

    let mut reports: Vec<Vec<i32>> = Vec::new();

    loop {
        inp.clear();
        let res = stdin
            .read_line(&mut inp)
            .expect("Error reading from stdin.");
        if res == 0 {
            break;
        }
        let report: Vec<i32> = inp
            .split_whitespace()
            .map(|x| x.parse().expect("Error parsing number"))
            .collect();
        reports.push(report);
    }

    let result = reports
        .into_iter()
        .map(|x| parse_report(x))
        .fold(0, |acc, b| acc + (if b { 1 } else { 0 }));

    println!("{}", result);
}
