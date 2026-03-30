// fn main() {
//     println!("Hello, world!");
//     let srr1: [_; 5];
//     srr1 = ['a', 'b', 'c', 'd', 'e'];
//     assert_eq!(std::mem::size_of_val(&srr1), 5);
//     println!("{:?}", srr1);
// }

// fn main() {
//     let (x, y, z);
//     (x, y, z) = (4, 3, 67);
//     println!("{} - {} - {}", x, y, z);
// }

// fn main() {
//     let age = 30;
//     let P = Person {
//         name: String::from("Yan"),
//         age,
//         hobby: None,
//     };
// }

// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

// fn main() {
//     let v: Point = Point(0, 127, 255);
//     check_color(v);

//     println!("Success!! ");
// }
// fn check_color(p: Point) {
//     let Point(x, _, z) = p;

//     assert_eq!(x, 0);
//     assert_eq!(p.1, 127);
//     assert_eq!(z, 255);
// }
// struct Person {
//     name: String,
//     age: u8,
//     hobby: Option<String>,
// }

// #[derive(Debug)]
// struct User {
//     email: String,
//     username: String,
//     active: bool,
//     sign_in_count: u32,
// }
// fn main() {
//     let u1: User = User {
//         email: String::from("someone@example.com"),
//         username: String::from("sunface"),
//         active: true,
//         sign_in_count: 1,
//     };
//     let u2 = set_email(u1);
//     // println!("{:?}", u1);
// }
// fn set_email(u: User) -> User {
//     User {
//         email: String::from("contact@gmail.com"),
//         ..u
//     }
// }
#[derive(Debug)]
struct Person {
    name: String,
    age: Box<u8>,
}
fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    let Person { name, ref age } = person;
    println!("The person's age is {}", person.age);
    println!("The person's name is {}", name)
}
