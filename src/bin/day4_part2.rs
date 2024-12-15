use std::io;

const XMAS: &[u8] = b"MAS";

fn diag(arr: &Vec<Vec<u8>>) -> i32 {
    let mut res = 0;
    let xmas = XMAS.to_vec();
    let mut xmasr = XMAS.to_vec();
    xmasr.reverse();
    let permutations = [
        (xmas.clone(), xmas.clone()),
        (xmasr.clone(), xmas.clone()),
        (xmas.clone(), xmasr.clone()),
        (xmasr.clone(), xmasr.clone()),
    ];
    for i in 0..(arr.len() - 2) {
        for j in 0..(arr[i].len() - 2) {
            for p in permutations.iter() {
                let mut mask = true;
                for k in 0..3 {
                    mask = mask && arr[i + k][j + k] == p.0[k];
                    mask = mask && arr[i + 2 - k][j + k] == p.1[k];
                }
                if mask {
                    res += 1;
                }
            }
        }
    }
    res
}

pub fn main() {
    let stdin = io::stdin();
    let mut buf = String::new();
    let mut arr: Vec<Vec<u8>> = Vec::new();
    loop {
        buf.clear();
        let res = stdin.read_line(&mut buf);
        if let Ok(0) = res {
            break;
        }
        arr.push(buf.trim_end().as_bytes().to_vec());
    }

    println!(
        "{}",
        diag(&arr)
    );
}
