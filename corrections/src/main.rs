#![allow(warnings)]
use std::cmp;
use std::io;
mod test;
fn main() {
    println!(
        "Magnitude of a unit vector: {}",
        magnitude(&[0.0, 1.0, 0.0])
    );

    let mut v = [1.0, 2.0, 9.0];
    println!("Magnitude of {v:?}: {}", magnitude(&v));
    normalize(&mut v);
    println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));
}

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut result = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            result[j][i] = matrix[i][j];
        }
    }
    result
}
fn compare() {
    loop {
        let mut input = String::new();
        println!("Give a number: 0 - 100: ");
        io::stdin().read_line(&mut input).unwrap();

        let score: isize = match input.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Error: {}", e);
                return;
            }
        };

        if score >= 1 && score <= 100 {
            print!("Your grade is\t");
            if score >= 80 {
                println!("A");
            } else if score >= 70 {
                println!("B");
            } else if score >= 60 {
                println!("C");
            } else if score >= 50 {
                println!("D");
            } else {
                println!("F")
            }

            let mut input = String::new();
            println!("Give a second number to calculate the max");

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read Input");

            let number: isize = input.trim().parse().expect("Input must be a number");
            let mut max = cmp::max(number, score);
            // max = number.max(score);
            println!("The maximum between {} and {} is {}", score, number, max);

            println!("Give a third number");
            input.clear(); // clear the input first
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let number: isize = input.trim().parse().expect("Input must be a number");
            max = max.max(number);
            println!("The maximum now is {}", max);

            break;
        }

        println!("You should give a number between 0 and 100\n\n")
    }
}

fn matrix_transpose() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];
    println!("Original:");
    for row in matrix {
        println!("{row:?}");
    }
    let transposed = transpose(matrix);
    println!("\nTransposed:");
    for row in transposed {
        println!("{row:?}");
    }
    println!("{:?}", transposed);
}

fn magnitude(v: &[f64]) -> f64 {
    let mut sum = 0.0;
    for i in v {
        sum += i * i;
    }
    sum.sqrt()
}
fn normalize(v: &mut [f64; 3]) {
    let mag = magnitude(v);
    for val in v {
        *val /= mag;
    }
}
