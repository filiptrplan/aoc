use crate::Direction::*;
use std::io;

#[derive(PartialEq, Clone)]
enum Direction {
    Inc,
    Dec,
    Neut,
    Unk,
}

fn parse_report_ignore(report: &Vec<i32>, ignore_idx: usize) -> bool {
    let mut report_clone = report.clone();
    report_clone.remove(ignore_idx);
    parse_report(&report_clone)
}

fn parse_all(report: &Vec<i32>) -> bool{
    let mut res = parse_report(&report);
    for i in 0..report.len() {
        print!("{}", i);
        res = res || parse_report_ignore(&report, i)
    }
    println!("{:?}", report);
    res
}

fn parse_report(report: &Vec<i32>) -> bool {
    let mut directions: Vec<Direction> = Vec::new();
    for i in 1..report.len() {
        let direction = if report[i] > report[i - 1] {
            Inc
        } else if report[i] < report[i - 1] {
            Dec
        } else if report[i] == report[i - 1] {
            Neut
        } else {
            Unk
        };
        directions.push(direction);
    }
    let ref_dir = &directions[0];
    for i in 0..directions.len() {
        if directions[i] != *ref_dir {
            return false;
        }
    }
    for i in 1..report.len() {
        if (report[i] - report[i - 1]).abs() > 3 || (report[i] - report[i - 1]).abs() < 1 {
            return false;
        }
    }

    return true;
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
        .map(|x| parse_all(&x))
        .fold(0, |acc, b| acc + (if b { 1 } else { 0 }));

    println!("{}", result);
}
