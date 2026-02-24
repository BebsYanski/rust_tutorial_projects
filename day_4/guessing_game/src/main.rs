use rand::random_range; // Used to generate Random numbers
use std::cmp::Ordering;
use std::io; // Used for user input // Used to compare numbers, to check if greater than, less, or equal
fn main() {
    println!("Hello, welcome to the Guessing Game!");
    println!("I'm thinking of a number between 1 to 100. Can you guess it?");

    // Generate a random number
    // No explicit Rng import is required for this new top-level function
    let secret_number = random_range(1..=100);
    // println!("secret: {}",secret_number)
    let mut guess_count: i32 = 0;

    loop {
        guess_count += 1;
        println!("Please input your guess:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small! Try again."),
            Ordering::Greater => println!("Too large! Try again."),
            Ordering::Equal => {
                println!(
                    "Congratulations, you got it right, and you did so in {} attempts",
                    guess_count
                );
                break;
            }
        }
    }
}
