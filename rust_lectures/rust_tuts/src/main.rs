fn main() {
    let age: i32 = 20;
    let animals = ["Goat", "Pig", "Dog", "Cat"];
    let ages: [i32; 10] = [20, 10, 30, 40, 50, 32, 33, 22, 24, 56];
    let my_vector = vec![1, 2, 3, 4, 5];
    println!("{}", age);
    println!("{:?}", animals);
    println!("{:?}", ages);
    println!("{:?}", my_vector);
    // let customer = Customer::create_customer(
    //     String::from("John"),
    //     20,
    //     String::from("john@example.com"),
    //     String::from("1234567890"),
    // );
    let customer = Customer {
        name: String::from("John"),
        age: 20,
        email: String::from("john@example.com"),
        mobile: String::from("1234567890"),
    };
    println!("{}", customer.name);
    println!("{}", customer.get_name());
    println!("{}", customer.age);
    println!("{}", customer.get_age());
    println!("{}", customer.email);
    println!("{}", customer.mobile);
}

struct Customer {
    name: String,
    age: i32,
    email: String,
    mobile: String,
}

impl Customer {
    // Method to create a new customer
    fn create_customer(name: String, age: i32, email: String, mobile: String) -> Customer {
        Customer {
            name,
            age,
            email,
            mobile,
        }
    }

    // Method to get the customer's name
    fn get_name(&self) -> &String {
        &self.name
    }

    // Method to get the customer's age
    fn get_age(&self) -> i32 {
        self.age
    }
}
