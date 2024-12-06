use std::io;

const XMAS: &[u8] = b"XMAS";

fn horiz(arr: &Vec<Vec<u8>>) -> i32 {
    let mut res = 0;
    let xmas = XMAS;
    let mut xmasr: Vec<u8> = XMAS.to_vec();
    xmasr.reverse();
    for i in 0..arr.len() {
        for j in 0..arr[i].len() - 3 {
            let sl = &arr[i][j..j + 4];
            if *sl == *xmas || *sl == *xmasr {
                res += 1;
            }
        }
    }
    res
}

fn transpose(arr: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut transposed = Vec::new();
    for j in 0..arr[0].len() {
        let mut row = Vec::new();
        for i in 0..arr.len() {
            row.push(arr[i][j]);
        }
        transposed.push(row);
    };
    transposed
}

fn rotate(arr: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut rotated = transpose(&arr);
    let n = arr.len();
    for i in 0..arr.len() {
        for j in 0..(n/2) {
            let a = rotated[i][j];
            rotated[i][j] = rotated[i][n-1-j];
            rotated[i][n-1-j] = a;
        }
    };
    rotated
}

fn vert(arr: &Vec<Vec<u8>>) -> i32 {
    horiz(&transpose(arr))
}

fn diag(arr: &Vec<Vec<u8>>) -> i32 {
    let mut res = 0;
    let xmas = XMAS;
    let mut xmasr = XMAS.to_vec();
    xmasr.reverse();
    for i in 0..(arr.len()-3) {
        for j in 0..(arr[i].len()-3) {
            let mut mask = true;
            for k in 0..4 {
                mask = mask && arr[i+k][j+k] == xmas[k]
            }
            if mask {
                res += 1;
            }
            mask = true;
            for k in 0..4 {
                mask = mask && arr[i+k][j+k] == xmas[3-k]
            }
            if mask {
                res += 1;
            }
        }
    };
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

    println!("{}", diag(&arr) + vert(&arr) + horiz(&arr) + diag(&rotate(&arr)));
}
