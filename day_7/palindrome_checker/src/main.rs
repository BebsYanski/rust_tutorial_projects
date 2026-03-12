// Palindrome checker
use std::cmp::Ordering;
use std::io;
use std::str::Chars;
fn main() {
    println!("Palindrome Checker!");

    println!("Enter word to check palindrome");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read line");

    let cleaned_input = clean_string(&input);

    if cleaned_input.is_empty() {
        println!("Please enter a valid non-empty string.");
        return;
    }

    if is_palindrome(&cleaned_input) {
        println!("{} is a palindrome", input.trim());
    } else {
        println!("{} is not a palindrome", input.trim());
    }

    let reversed = input
        .trim()
        .chars()
        .rev()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>();
    println!("{}", reversed);
    let reversed = reversed.to_lowercase();

    match cleaned_input.to_lowercase().cmp(&reversed) {
        Ordering::Equal => println!("Palindrome"),
        Ordering::Greater => println!("Bigger"),
        Ordering::Less => println!("Smaller"),
    }

    if cleaned_input.to_lowercase() == reversed.to_lowercase() {
        println!("Palindrome");
    } else {
        println!("Not Palindrome")
    }
}

fn clean_string(input: &str) -> String {
    input
        .chars()
        .filter(|c| c.is_alphanumeric()) // keep only letters and numbers
        .map(|c| c.to_lowercase().to_string()) //Convert to lowercase, and back to String
        .collect::<String>()
}

fn is_palindrome(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}
