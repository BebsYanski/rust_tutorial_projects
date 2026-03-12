use std::cmp;
use std::io;
fn main() {
    println!("Hello guys");

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

    // Compare
}
