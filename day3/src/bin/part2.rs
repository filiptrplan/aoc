use crate::Token::*;
use std::io;

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Num(i64),
    LeftParen,
    RightParen,
    Mul,
    Do,
    Dont,
    Comma,
    Other(u8),
}

fn tokenize(line: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let line_bytes = line.as_bytes();
    let mut i = 0;
    while i < line.len() {
        let c = line_bytes[i];
        match c {
            b'(' => tokens.push(LeftParen),
            b')' => tokens.push(RightParen),
            b',' => tokens.push(Comma),
            b'm' => {
                if i + 2 < line_bytes.len() && line_bytes[i..(i + 3)] == *("mul".as_bytes()) {
                    tokens.push(Mul);
                    i = i + 2;
                } else {
                    tokens.push(Other(b'm'));
                }
            }
            b'0'..=b'9' => {
                let start = i;
                i += 1;
                while i < line_bytes.len() && line_bytes[i].is_ascii_digit() {
                    i += 1;
                }
                let num: i64 = line[start..i].parse().unwrap();
                tokens.push(Num(num));
                continue;
            }
            b'd' => {
                if i + 4 < line_bytes.len() && line_bytes[i..(i + 5)] == *("don't".as_bytes()) {
                    tokens.push(Dont);
                    i = i + 4;
                } else if i + 1 < line_bytes.len() && line_bytes[i..(i + 2)] == *("do".as_bytes()) {
                    tokens.push(Do);
                    i = i + 1;
                } else {
                    tokens.push(Other(b'd'));
                }
            }
            c => {
                tokens.push(Other(c));
            }
        };
        i += 1;
    }
    tokens
}

fn parse(tokens: Vec<Token>) -> i64 {
    let mut res = 0;
    let mut i = 0;
    let mut enable = true;
    while i < tokens.len() {
        match tokens[i] {
            Mul => {
                if i + 5 < tokens.len() {
                    let expression = &tokens[i..i + 6];
                    if let [Mul, LeftParen, Num(a), Comma, Num(b), RightParen] = expression {
                        if enable {
                            res += a * b;
                        }
                        i += 4;
                    }
                }
            }
            Do => {
                if i + 2 < tokens.len() {
                    let expression = &tokens[i..i + 3];
                    if let [Do, LeftParen, RightParen] = expression {
                        enable = true;
                        i += 2;
                    }
                }
            }
            Dont => {
                if i + 2 < tokens.len() {
                    let expression = &tokens[i..i + 3];
                    if let [Dont, LeftParen, RightParen] = expression {
                        enable = false;
                        i += 2;
                    }
                }
            },
            _ => {}
        }
        i += 1;
    }
    res
}

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    loop {
        line = line.trim_end_matches('\n').to_owned();
        if let Ok(0) = stdin.read_line(&mut line) {
            break;
        }
    }
    println!("{} {}", line.len(), line.as_bytes().len());
    let tokens = tokenize(line.clone());
    // println!("{:?}", tokens);
    println!("{}", parse(tokens));
}
