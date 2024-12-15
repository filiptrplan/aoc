use std::{collections::HashMap, io, u32};

type Coord = (i128, i128);

#[derive(Debug, Clone, Copy)]
struct Machine {
    prize: Coord,
    a_button: Coord,
    b_button: Coord,
}

fn calc_price(button_presses: (i128, i128)) -> i128 {
    button_presses.0 * 3 + button_presses.1
}
fn find_prize(machine: &Machine) -> Option<i128> {
    let y = machine.prize.1 as f64;
    let x = machine.prize.0 as f64;
    let ax = machine.a_button.0 as f64;
    let ay = machine.a_button.1 as f64;
    let bx = machine.b_button.0 as f64;
    let by = machine.b_button.1 as f64;
    let b = (y - ((ay * x) / ax)) / ((-bx * ay) / ax + by);
    let a = (x - b * bx) / ax;

    let b128 = b.round() as i128;
    let a128 = a.round() as i128;

    if a128 * machine.a_button.0 + b128 * machine.b_button.0 != machine.prize.0
        || a128 * machine.a_button.1 + b128 * machine.b_button.1 != machine.prize.1
    {
        return None;
    }

    return Some(calc_price((a128, b128)));
}

fn main() {
    let stdin = io::stdin();
    let mut buf = String::new();

    let mut machines = Vec::new();
    let re = regex::RegexBuilder::new(r"X\+([0-9]+), Y\+([0-9]+).*X\+([0-9]+), Y\+([0-9]+)")
        .dot_matches_new_line(true)
        .build()
        .unwrap();
    let re_prize = regex::Regex::new(r"X=([0-9]+), Y=([0-9]+)").unwrap();
    loop {
        buf.clear();
        let res = stdin.read_line(&mut buf);
        if let Ok(0) = res {
            break;
        }
        let _ = stdin.read_line(&mut buf);
        let _ = stdin.read_line(&mut buf);
        let _ = stdin.read_line(&mut buf);
        if let Some(buttons) = re.captures(&buf) {
            // println!("{}, {:?}", buf, buttons);
            if let Some(prize) = re_prize.captures(&buf) {
                machines.push(Machine {
                    prize: (
                        prize[1].parse::<i128>().unwrap() + 10000000000000,
                        prize[2].parse::<i128>().unwrap() + 10000000000000,
                        // prize[1].parse::<i128>().unwrap(),
                        // prize[2].parse::<i128>().unwrap(),
                    ),
                    a_button: (buttons[1].parse().unwrap(), buttons[2].parse().unwrap()),
                    b_button: (buttons[3].parse().unwrap(), buttons[4].parse().unwrap()),
                });
            }
        }
    }
    println!(
        "{:?}",
        machines
            .iter()
            .map(|machine| { find_prize(machine).unwrap_or(0) })
            .fold(0, |a, b| a + b)
    );
}
