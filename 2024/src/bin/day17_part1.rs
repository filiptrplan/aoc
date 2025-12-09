use std::io;
use std::io::Read;

use crate::Opcode::*;
use crate::Operand::*;

enum Operand {
    Lit(i32),
    A,
    B,
    C,
}
enum Opcode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

struct Machine {
    a: i32,
    b: i32,
    c: i32,
    instructions: Vec<u8>,
    ip: usize,
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0 => Adv,
            1 => Bxl,
            2 => Bst,
            3 => Jnz,
            4 => Bxc,
            5 => Out,
            6 => Bdv,
            7 => Cdv,
            _ => Cdv,
        }
    }
}

impl From<u8> for Operand {
    fn from(value: u8) -> Self {
        match value {
            0..=3 => Lit(value as i32),
            4 => A,
            5 => B,
            6 => C,
            _ => C,
        }
    }
}

impl From<String> for Machine {
    fn from(value: String) -> Self {
        let lines = value.split_terminator("\n").collect::<Vec<&str>>();
        Machine {
            a: lines[0].split_ascii_whitespace().collect::<Vec<&str>>()[2]
                .parse()
                .unwrap(),
            b: lines[1].split_ascii_whitespace().collect::<Vec<&str>>()[2]
                .parse()
                .unwrap(),
            c: lines[2].split_ascii_whitespace().collect::<Vec<&str>>()[2]
                .parse()
                .unwrap(),
            instructions: lines[4]
                .split_ascii_whitespace()
                .rev()
                .next()
                .unwrap()
                .split_terminator(",")
                .map(|x| x.parse().unwrap())
                .collect(),
            ip: 0,
        }
    }
}

impl Machine {
    fn get(&self, operand: Operand) -> i32 {
        match operand {
            Lit(a) => a,
            A => self.a,
            B => self.b,
            C => self.c,
        }
    }
    fn run(&mut self) -> Vec<i32> {
        let mut output: Vec<i32> = Vec::new();

        while self.ip < self.instructions.len() {
            let op = Opcode::from(self.instructions[self.ip]);
            let combo = self.get(Operand::from(self.instructions[self.ip + 1]));
            let lit: i32 = self.instructions[self.ip + 1] as i32;

            match op {
                Adv => {
                    self.a = self.a / (1 << combo);
                }
                Bxl => {
                    self.b = self.b ^ lit;
                }
                Bst => self.b = combo & ((1 << 3) - 1),
                Jnz => {
                    if self.a != 0 {
                        self.ip = lit as usize;
                        continue;
                    }
                }
                Bxc => {
                    self.b = self.b ^ self.c;
                }
                Out => {
                    output.push(combo & ((1 << 3) - 1));
                }
                Bdv => {
                    self.b = self.a / (1 << combo);
                }
                Cdv => {
                    self.c = self.a / (1 << combo);
                }
            }

            self.ip += 2;
        }

        output
    }
}

fn main() {
    let mut buf = Vec::new();
    let mut stdin = io::stdin();

    let _ = stdin.read_to_end(&mut buf);
    let buf_str = String::from_utf8_lossy(&buf).into_owned();

    let result = Machine::from(buf_str).run();
    let result_str: Vec<String> = result.iter().map(|&num| num.to_string()).collect();
    println!("{}", result_str.join(","));
}
