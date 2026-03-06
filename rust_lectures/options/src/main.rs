fn main() {
    let age = 10;

    let mut number: Option<i32> = Some(30); // some_number is a function that returns an Option<i32> which is a type that can be either Some(i32) or None. It is a type that is used to handle the possibility of a value being present or absent.
    match number {
        Some(n) => println!("Number is {}", n),
        None => println!("Number is not present"),
    }

    number = None;
    match number {
        Some(n) => println!("Number is {}", n),
        None => println!("Number is not present"),
    }

    let num1 = 40;
    let num2 = 20;
    let result = divide(num1, num2);
    match result {
        Some(n) => println!("Result of {} / {} is {}", num1, num2, n),
        None => println!("Division by zero is not allowed"),
    }

    let result = division(num1, num2);
    match result {
        Ok(n) => println!("Result of {} / {} is {}", num1, num2, n),
        Err(e) => println!("{}", e),
    }
}

fn get_number() -> Option<i32> {
    Some(30)
}

fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

fn division(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Division by zero is not allowed"))
    } else {
        Ok(a / b)
    }
}
