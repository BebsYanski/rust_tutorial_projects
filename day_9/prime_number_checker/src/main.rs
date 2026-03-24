use std::io;
fn main() {
    println!("Give number to check");
    let number = match get_input() {
        Some(value) => value,
        None => {
            println!("Error");
            return;
        }
    };

    println!("Prime: {}", check_prime(number))
}

fn check_prime(n: u32) -> bool {
    let mut prime = true;
    for num in 2..(n - 1) {
        if n % num == 0 {
            prime = false;
        }
    }
    prime
}

fn get_input() -> Option<u32> {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    match input.trim().parse::<u32>() {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}
