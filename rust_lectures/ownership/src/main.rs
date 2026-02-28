// Ownership, Borrowing, References
//  Ownership
// ----------------------
// C, C++ --> Memory Management Control Issue
// Garbage Collector solved this issue, but created a new issue --> Slow Performance:
// [stopping/Resuming the program]

// Ownership Rules
// 1. Each value in rust has an owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.

// // Example: Each value in rust has an owner.
// fn main() {
// let s1 = String::from("RUST");
// let len = calculate_length(&s1);
// println!("{}",len);
// }

// fn calculate_length(s: &String) -> usize{
//     s.len()
// }


// // Example: There can only be one owner at a time.
// fn main() {
//     let s1 = String::from("RUST");
//     let s2 = s1; 
//     let len = calculate_length(&s1);
//     println!("{}",len);
//     }
    
//     fn calculate_length(s: &String) -> usize{
//         s.len()
//     }


// 3. When the owner goes out of scope, the value will be dropped.
// fn main() {
//     let s1 = String::from("RUST");
//     let len = calculate_length(&s1);
//     println!("The length of {} is {}",s1,len);
//     }

//     // s1 goes out of scope and its value will be dropped
//     fn printLost(s:&String){
// println!("{}",&s1);
//     }
    
//     fn calculate_length(s: &String) -> usize{
//         s.len()
//     }

//  references and Borrowing
// Safety and Performance
// Borrowing and references are powerful concepts in rust.
// Safety is the prevention of some of the errors that commonly occur in other languages

// Understand References
// Enables you to borrow values, without taking from the owner
// Mutable and imutable refferences
fn main(){
    let mut _x: i32 = 5;

    
    let _r =  &mut _x;
    *_r += 1;
    *_r -= 3;

    println!("Value of _x: {}",_x);

    let a = 6;
    let b = &a;
    println!("Value of b: {}",*b);


}