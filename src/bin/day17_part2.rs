use std::io;
use std::io::Read;

use indicatif::ProgressBar;

use crate::Opcode::*;
use crate::Operand::*;

enum Operand {
    Lit(u128),
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
    a: u128,
    b: u128,
    c: u128,
    init: (u128, u128, u128),
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
            0..=3 => Lit(value as u128),
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
        let a = lines[0].split_ascii_whitespace().collect::<Vec<&str>>()[2]
            .parse()
            .unwrap();
        let b = lines[1].split_ascii_whitespace().collect::<Vec<&str>>()[2]
            .parse()
            .unwrap();
        let c = lines[2].split_ascii_whitespace().collect::<Vec<&str>>()[2]
            .parse()
            .unwrap();
        Machine {
            a,
            b,
            c,
            init: (a, b, c),
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
    fn get(&self, operand: Operand) -> u128 {
        match operand {
            Lit(a) => a,
            A => self.a,
            B => self.b,
            C => self.c,
        }
    }
    fn run_and_reset(&mut self, init_a: u128) -> bool {
        let mut i = 0;
        self.a = init_a;
        self.b = self.init.1;
        self.c = self.init.2;
        self.ip = 0;

        while self.ip < self.instructions.len() {
            let op = Opcode::from(self.instructions[self.ip]);
            let combo = self.get(Operand::from(self.instructions[self.ip + 1]));
            let lit: u128 = self.instructions[self.ip + 1] as u128;

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
                    let out = (combo as u8) & ((1 << 3) - 1);
                    if i >= self.instructions.len() || self.instructions[i] != out {
                        return false;
                    }
                    i += 1;
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
        return i == self.instructions.len();
    }
}

fn main() {
    let mut buf = Vec::new();
    let mut stdin = io::stdin();

    let _ = stdin.read_to_end(&mut buf);
    let buf_str = String::from_utf8_lossy(&buf).into_owned();
    let mut machine = Machine::from(buf_str);

    let num = 1 << (3 * 16);
    let bar = ProgressBar::new((1 << (3 * 16)) - (1 << (3 * 15)));

    for i in (1 << (3 * 15))..=(1 << (3 * 16)) {
        bar.inc(1);
        if i / num == 0 {
            if machine.run_and_reset(i) {
                println!("{}", i);
                break;
            }
        }
    }
}
