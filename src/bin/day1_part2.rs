use std::io;

fn main() {
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    let mut inp = String::new();

    loop {
        inp.clear();
        let res = io::stdin().read_line(&mut inp);
        match res {
            Ok(0) | Err(_) => break,
            Ok(_) => {
                let split_string: Vec<&str> = inp.split_whitespace().collect();
                list1.push(split_string[0].parse().expect("Failed to parse string 1"));
                list2.push(split_string[1].parse().expect("Failed to parse string 2"));
            }
        }
    }

    let distance = list1
        .into_iter()
        .map(|x| {
            let count = list2.iter().fold(0, |acc, x2| if x == *x2 { 1 + acc } else { acc } );
            x * count
        })
        .fold(0, |a,b| a+b);

    println!("{}", distance);
}
