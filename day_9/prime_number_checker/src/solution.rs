use std::io;

use crate::is_prime;
pub fn check_prime(n: u32) -> bool {
    let mut prime = true;
    for num in 2..(n - 1) {
        if n % num == 0 {
            prime = false;
        }
    }
    prime
}

pub fn get_input() -> Option<u32> {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    match input.trim().parse::<u32>() {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}

pub fn get_all_prime(n: u32) -> Vec<u32> {
    let mut sequence: Vec<u32> = Vec::new();

    for num in 2..n {
        if is_prime(num) {
            sequence.push(num);
        }
    }

    sequence
}
