// Store 5 numbers in an array
// Print them
// Calculate sum
use std::io;
fn main() {
    // Ask the user how many numbers they want to enter
    let mut input = String::new();

    println!("How many numbers do you want to enters?");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Convert the input to a usize
    let input: usize = input.trim().parse().expect("Please type a number!");

    // Create a vector to store the numbers
    let mut numbers: Vec<i32> = Vec::new();

    // Ask the user to enter the numbers
    for _ in 0..input {
        let mut number = String::new();
        println!("Enter a number: ");
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");
        let number: i32 = number.trim().parse().expect("Please type a number!"); // Convert the input to a i32
        numbers.push(number); // Add the number to the vector
    }

    // Print the numbers
    println!("Numbers: {:?}", numbers);

    // Calculate the sum of the numbers
    // let sum: i32 = numbers.iter().sum();
    let sum = sum_vector(&numbers);
    println!("Sum: {}", sum);
}

fn sum_vector(numbers: &Vec<i32>) -> i32 {
    let mut sum: i32 = 0;
    for number in numbers {
        sum += number;
    }
    sum
}
