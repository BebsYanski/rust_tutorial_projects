use std::io; // Import the standard input/output library which allows us to read user input from the console.
fn main() {
    println!("Hello, Temperature Converter!");
    println!("1: Celsius to Fahrenheit");
    println!("2: Fahrenheit to Celsius");
    println!("Please enter your choice (1 or 2):");

    let mut choice = String::new(); // Create a mutable String variable to store the user's choice.
    io::stdin().read_line(&mut choice).expect("Failed to read line"); // Read the user's input from the console and store it in the 'choice' variable. If reading fails, it will panic with the message "Failed to read line".

    let choice: u32 = match choice.trim().parse() { // Trim whitespace from the input and attempt to parse it as an unsigned 32-bit integer.
        Ok(num) => num, // If parsing is successful, store the number in 'choice'.
        Err(_) => {
            println!("Invalid input. Please enter a number."); // If parsing fails, print an error message.
            return; // Exit the program early if the input is invalid.
        }
    };

    if choice == 1 {
        println!("Enter temperature in Celsius:");
        let mut celsius = String::new(); // Create a mutable String variable to store the user's input for Celsius.
        io::stdin().read_line(&mut celsius).expect("Failed to read line"); // Read the user's input for Celsius.

        let celsius: f64 = match celsius.trim().parse() { // Trim whitespace and attempt to parse the input as a floating-point number.
            Ok(num) => num, // If parsing is successful, store the number in 'celsius'.
            Err(_) => {
                println!("Invalid input. Please enter a valid number."); // If parsing fails, print an error message.
                return; // Exit the program early if the input is invalid.
            }
        };

        let fahrenheit = celsius_to_fahrenheit(celsius); // Call the function to convert Celsius to Fahrenheit.
        println!("{} °C is equal to {} °F", celsius, fahrenheit); // Print the result of the conversion.
    } else if choice == 2 {
       fahrenheit_to_celsius(celsius); // Call the function to convert Fahrenheit to Celsius.
       
    } else {
        println!("Invalid choice. Please enter 1 or 2."); // If the user's choice is not 1 or 2, print an error message.
    }
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0 // Function to convert Celsius to Fahrenheit.
}

fn fahrenheit_to_celsius(fahrenheit: f64)  {
    println!("Enter temperature in Fahrenheit:");
    let mut fahrenheit = String::new(); // Create a mutable String variable to store the user's input for Fahrenheit.
    io::stdin().read_line(&mut fahrenheit).expect("Failed to read line"); // Read the user's input for Fahrenheit.

    let fahrenheit: f64 = match fahrenheit.trim().parse() { // Trim whitespace and attempt to parse the input as a floating-point number.
        Ok(num) => num, // If parsing is successful, store the number in 'fahrenheit'.
        Err(_) => {
            println!("Invalid input. Please enter a valid number."); // If parsing fails, print an error message.
            return; // Exit the function early if the input is invalid.
        }
    };

    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0; // Convert Fahrenheit to Celsius using the formula.
    println!("{:.2} °F is equal to {:.2} °C", fahrenheit, celsius); // Print the result of the conversion.
}