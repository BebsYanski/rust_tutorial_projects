// Menu Driven Program
use std::io;

fn main() {
    loop {
        // loop to keep the program running
        // Show menu
        println!("1. Add");
        println!("2. Subtract");
        println!("3. Multiply");
        println!("4. Divide");
        println!("5. Exit");

        // Get user input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // convert input to usize
        let input: usize = input.trim().parse().expect("Please type a number!");

        if input == 5 {
            println!("Exiting...");
            break;
        }
        // Collect first number
        println!("Enter first number: ");
        let mut first_number = String::new();
        io::stdin()
            .read_line(&mut first_number)
            .expect("Failed to read line");
        let first_number: i32 = first_number.trim().parse().expect("Please type a number!");

        // Collect second number
        println!("Enter second number: ");
        let mut second_number = String::new();
        io::stdin()
            .read_line(&mut second_number)
            .expect("Failed to read line");
        let second_number: i32 = second_number.trim().parse().expect("Please type a number!");

        match input {
            1 => {
                let result = add(first_number, second_number);
                println!("Result: {}", result);
            }
            2 => {
                let result = subtract(first_number, second_number);
                println!("Result: {}", result);
            }
            3 => {
                let result = multiply(first_number, second_number);
                println!("Result: {}", result);
            }
            4 => {
                let result = divide(first_number, second_number);
                println!("Result: {}", result);
            }
            5 => {
                println!("Exiting...");
                return;
            }
            _ => println!("Invalid input"),
        }
    }
}

fn add(first_number: i32, second_number: i32) -> i32 {
    first_number + second_number
}

fn subtract(first_number: i32, second_number: i32) -> i32 {
    first_number - second_number
}

fn multiply(first_number: i32, second_number: i32) -> i32 {
    first_number * second_number
}

fn divide(first_number: i32, second_number: i32) -> i32 {
    first_number / second_number
}
