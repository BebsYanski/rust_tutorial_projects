use std::io;

fn main(){
    println!("Simple Calculator");
    print!("Available Operations: +, -, *, /");
    println!("Enter your Expression (e.g., 5 + 3):");

    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)// read a line of input
    .expect("Failed to read input"); // Handle errors

    //Tokenize the input: means we partition the input into its different components.
    let tokens: Vec<&str> = input.trim().split_whitespace().collect();

    if tokens.len() != 3 {
        print!("Invalid input. Please follow the format: number operator number (e.g., 5 + 3)");
        return;
    }

}