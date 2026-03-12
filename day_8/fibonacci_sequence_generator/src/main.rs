use std::io;
fn main() {
    println!("Fibonacci Sequence Generator");
    println!("Enter the number of terms you want to generate");

    let num_terms = match get_input_as_u32() {
        Some(val) => val,
        None => {
            println!("Invalid input. Please enter a positive integer");
            return;
        }
    };

    if num_terms == 0 {
        println!("Number of terms must be greater than zero.");
        return;
    }

    let sequence = fibo(num_terms);
    println!("Fibonacci Sequence ({} terms): {:?}", num_terms, sequence);
}

fn fibo(num: u32) -> Vec<u64> {
    let mut sequence = Vec::new();

    if num >= 1 {
        sequence.push(0);
    }
    if num >= 2 {
        sequence.push(1);
    }

    for i in 2..num {
        let next = sequence[i as usize - 1] + sequence[i as usize - 2];
        sequence.push(next);
    }
    sequence

    // return arr;
}
fn get_input_as_u32() -> Option<u32> {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    match input.trim().parse::<u32>() {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}
