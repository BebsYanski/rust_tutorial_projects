use std::io;
fn main() {
    println!("Enter a number:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Convert the input to a i32
    let number = input
        .trim()
        .parse::<i32>()
        .expect("Invalid input, Please enter a number");

    println!("You entered: {}", number);
}
