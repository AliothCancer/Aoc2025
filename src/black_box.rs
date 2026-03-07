#![allow(unused)]
use std::fs::read_to_string;
use std::hint::black_box;

pub fn main_black_box() {
    let file = read_to_string("./src/input_big.txt").unwrap();
    let lines: Vec<String> = file.lines().map(String::from).collect();

    let t1 = std::time::Instant::now();
    let total: u32 = lines.into_iter().map(|x| black_box(get_max_claude(black_box(&x)))).sum();
    let duration = t1.elapsed().as_millis();
    println!("sol: {total} duration: {} ms", duration);
}

fn get_max_claude(line: &str) -> u32 {
    let digits: Vec<u32> = line.bytes().map(|b| (b - b'0') as u32).collect();
    let n = digits.len();

    let mut max_val = 0u32;

    for i in 0..n - 1 {
        let base = digits[i] * 10;
        for j in i + 1..n {
            let pair = base + digits[j];
            if pair > max_val {
                max_val = pair;
            }
        }
    }

    max_val
}