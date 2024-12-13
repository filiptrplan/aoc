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

fn gcd(a_in: i128, b_in: i128) -> (i128, i128, i128) {
    let mut r = (a_in, b_in);
    let mut s = (1, 0);
    let mut t = (0, 1);

    while r.1 != 0 {
        let q = r.0 / r.1;
        r = (r.1, r.0 - q * r.1);
        s = (s.1, s.0 - q * s.1);
        t = (t.1, t.0 - q * t.1);
    }
    (r.0, s.0, t.0)
}

fn find_prize(machine: &Machine) -> Option<i128> {
    let gcd_x = gcd(machine.a_button.0, machine.b_button.0);
    if machine.prize.0 % gcd_x.0 != 0 {
        return None;
    }
    let gcd_y = gcd(machine.a_button.1, machine.b_button.1);
    if machine.prize.1 % gcd_y.0 != 0 {
        return None;
    }

    let mut solution_x = (
        gcd_x.1 * (machine.prize.0 / gcd_x.0),
        gcd_x.2 * (machine.prize.0 / gcd_x.0),
    );
    let mut solution_y = (
        gcd_y.1 * (machine.prize.1 / gcd_y.0),
        gcd_y.2 * (machine.prize.1 / gcd_y.0),
    );

    if solution_x.0 < 0 && solution_x.1 < 0 {
        return None;
    }
    if solution_y.0 < 0 && solution_y.1 < 0 {
        return None;
    }

    let delta = (machine.b_button.0 / gcd_x.0, machine.a_button.0 / gcd_x.0);

    let k_min = ((-solution_x.0 as f64) / delta.0 as f64).ceil() as i128;
    let k_max = (solution_x.1 as f64 / delta.1 as f64).floor() as i128;

    if k_min > k_max {
        return None;
    }

    solution_x.0 += delta.0 * k_min;
    solution_x.1 -= delta.1 * k_min;

    let mut min_cost: Option<i128> = None;

    // Generate all non-negative solutions for x
    for k_x in k_min..=k_max {
        let x = solution_x.0 + delta.0 * k_x;
        let y = solution_x.1 - delta.1 * k_x;

        if x >= 0 && y >= 0 {
            // Generate all non-negative solutions for y
            let delta_y = (
                machine.b_button.1 / gcd_y.0,
                machine.a_button.1 / gcd_y.0,
            );
            let k_min_y = ((-solution_y.0) as f64 / delta_y.0 as f64).ceil() as i128;
            let k_max_y = (solution_y.1 as f64 / delta_y.1 as f64).floor() as i128;

            for k_y in k_min_y..=k_max_y {
                let x_y = solution_y.0 + delta_y.0 * k_y;
                let y_y = solution_y.1 - delta_y.1 * k_y;

                if x_y >= 0 && y_y >= 0 {
                    let total_presses = (x + x_y, y + y_y);
                    let cost = calc_price(total_presses);
                    min_cost = Some(min_cost.map_or(cost, |min| min.min(cost)));
                }
            }
        }
    }

    

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
                        // prize[1].parse::<i128>().unwrap() + 10000000000000,
                        // prize[2].parse::<i128>().unwrap() + 10000000000000,
                        prize[1].parse::<i128>().unwrap(),
                        prize[2].parse::<i128>().unwrap(),
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
