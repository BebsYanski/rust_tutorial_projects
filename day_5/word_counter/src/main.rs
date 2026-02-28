use std::env; // Gives us access to command line arguments
use std::fs::File; // Opens and reads files
use std::io::{self, Read}; // Help reading files, and errors

fn main() {
    // collect command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: cargo run <file_path>");
        return;
    }

    let file_path = &args[1];
    println!("Reading file: {}", file_path);

    // Read the file contents
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => {
            println!("Error opening file: {}", err);
            return;
        }
    };

    let mut contents = String::new();
    if let Err(err) = file.read_to_string(&mut contents) {
        println!("Error reading file: {}", err);
        return;
    }

    // Count words
    let word_count = count_words(&contents);
    let char_count = count_chars(&contents);
    let line_count = count_lines(&contents);

    println!("Character Count: {}", char_count);
    println!("Word Count: {}", word_count);
    println!("Line Count: {}", line_count);
}

fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

fn count_chars(text: &str) -> usize {
    // text.split("").count()
    // text.len()
    text.chars().count()
}
fn count_lines(text: &str) -> usize {
    // text.split("\n").count()
    text.lines().count()
}
