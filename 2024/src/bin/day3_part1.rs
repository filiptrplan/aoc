use crate::Token::*;
use std::io;

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Num(i64),
    LeftParen,
    RightParen,
    Mul,
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
                println!("{}", &line[start..i]);
                let num: i64 = line[start..i].parse().unwrap();
                tokens.push(Num(num));
                continue;
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
    while i < tokens.len() {
        if let Mul = tokens[i] {
            if i + 5 < tokens.len() {
                let expression = &tokens[i..i + 6];
                if let [Mul, LeftParen, Num(a), Comma, Num(b), RightParen] = expression {
                    res += a * b;
                    i += 4;
                }
            }
        }
        i += 1;
    }
    res
}

fn reconstruct(tokens: Vec<Token>) -> String {
    let mut result = String::new();
    for token in tokens {
        match token {
            Token::Num(n) => result.push_str(&n.to_string()),
            Token::LeftParen => result.push('('),
            Token::RightParen => result.push(')'),
            Token::Mul => result.push_str("mul"),
            Token::Comma => result.push(','),
            Token::Other(c) => result.push(c as char),
        }
    }
    result
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
    let reconstructed = reconstruct(tokens.clone());
    println!("{:?}", reconstructed == line);
    if reconstructed != line {
        // println!("Original: {}", line);
        for (i, (orig, recon)) in line.chars().zip(reconstructed.chars()).enumerate() {
            if orig != recon {
            println!("Difference at index {}: original '{}', reconstructed '{}'", i, orig, recon);
            break;
            }
        }
        // println!("Reconstructed: {}", reconstructed);
    }
    println!("{}", parse(tokens));
}
