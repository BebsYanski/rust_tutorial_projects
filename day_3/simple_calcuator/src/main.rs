use std::io;

fn main(){
    println!("Simple Calculator");
    println!("Available Operations: +, -, *, /");
    println!("Enter your Expression (e.g., 5 + 3):\t");

    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)// read a line of input
    .expect("Failed to read input"); // Handle errors

    //Tokenize the input: means we partition the input into its different components.
    let tokens: Vec<&str> = input.trim().split_whitespace().collect();

    if tokens.len() != 3 {
        println!("Invalid input. Please follow the format: number operator number (e.g., 5 + 3)");
        return;
    }

    // We now extract the individual tokens, and convert the strings to numbers, leaving the operators
    let num1:f64 = match tokens[0].parse(){
        Ok(n)=>n,
        Err(_)=>{
            println!("Invalid first number");
            return;
        }
    };
    
    let operator = tokens[1];

     let num2:f64 = match tokens[2].parse(){
        Ok(n)=>n,
        Err(_)=>{
            println!("Invalid first number");
            return;
        }
    };

    let result = match operator{
        "+" => add(num1 , num2),
        "-" => subtract(num1 , num2),
        "*" => multiply(num1 , num2),
        "/" => divide(num1 , num2),
        _ => {
            println!("Invalid operator. Use +, -, *, or /.");
            return
        }

    };

    print!("Result: {:.2}",result)

}

fn add(num1:f64,num2:f64) -> f64{
return num1 + num2;
}

fn subtract(num1:f64,num2:f64) -> f64{
    num1 - num2
}
fn multiply(num1:f64,num2:f64) -> f64{
return num1 * num2;
}
fn divide(num1:f64,num2:f64) -> f64{
    if num2 == 0.0 {
        println!("Division by zero (0) is not allowed.");
        std::process::exit(1);
    }
    num1 / num2
}