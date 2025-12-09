use std::{collections::HashMap, io};

type Coord = (u32, u32);

#[derive(Debug, Clone, Copy)]
struct Machine {
    prize: Coord,
    a_button: Coord,
    b_button: Coord,
}

fn calc_price(button_presses: (u32, u32)) -> u32 {
    button_presses.0 * 3 + button_presses.1
}

fn find_prize(
    machine: &Machine,
    cur_pos: Coord,
    memo: &mut HashMap<Coord, Option<u32>>,
    button_presses: (u32, u32),
) -> Option<u32> {
    if cur_pos == machine.prize {
        return Some(calc_price(button_presses));
    }
    if button_presses.0 > 100 || button_presses.1 > 100 {
        return None;
    }

    if memo.contains_key(&cur_pos) {
        return *memo.get(&cur_pos).unwrap();
    }

    let res_a = find_prize(
        machine,
        (
            cur_pos.0 + machine.a_button.0,
            cur_pos.1 + machine.a_button.1,
        ),
        memo,
        (button_presses.0 + 1, button_presses.1),
    );
    let res_b = find_prize(
        machine,
        (
            cur_pos.0 + machine.b_button.0,
            cur_pos.1 + machine.b_button.1,
        ),
        memo,
        (button_presses.0, button_presses.1 + 1),
    );


    let res = match (res_a, res_b) {
        (None, None) => None,
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (Some(a), Some(b)) => Some(if a < b {a} else {b})
    };
    memo.insert(cur_pos, res);
    return res;
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
                    prize: (prize[1].parse().unwrap(), prize[2].parse().unwrap()),
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
            .map(|machine| {
                let mut memo = HashMap::new();
                return find_prize(machine, (0, 0), &mut memo, (0, 0)).unwrap_or(0);
            })
            .fold(0, |a, b| a + b)
    );
}
