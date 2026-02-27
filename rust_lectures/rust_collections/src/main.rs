fn main() {
    let numbers = [1, 2, 3, 4, 5];
    println!("{:?}", numbers);
    let fruits: [&str; 4] = ["Apple", "Orange", "Banana", "Mango"];
    println!("{:?}", fruits);

    for i in fruits {
        println!("{}", i);
    }
    let new_fruits = [
        "Apple".to_string(),
        "Banana".to_string(),
        "Orange".to_string(),
        "Mango".to_string(),
    ];
    let fruits: [String; 4] = new_fruits;
    println!("{:?}", fruits);
    println!("{}", fruits[0]);
    // Tuples
    let my_tuple: (&str, i32, bool) = ("Yan", 2, false);
    println!("{:?}", my_tuple);

    let my_mix_tuple = ([1, 2, 3], "Kratos", true);
    println!("{:?}", my_mix_tuple);

    // Slices: Dynamically sized in a contiguous sequence of memory
    let number_slices: &[i32] = &[1, 2, 3, 4];
    println!("{:?}", number_slices);

    let string_slice: &[&str] = &["Goat", "Pig"];
    println!("{:?}", string_slice);

    let animal_slice: &[String] = &["Goat".to_string(), "Pig".to_string()];
    println!("{:?}", animal_slice);

    let animal_slice: &[&String] = &[&"Goat".to_string(), &"New".to_string()];
    println!("{:?}", animal_slice);

    // Strings vs String Slices (&str)
    // Strings: [mutable,growable,owned string type].
    // stored dynamically on the heap, hence slower
    let mut name: String = String::from("Yannick");
    println!("{}", name);
    name.push_str(" Bebongnchu");

    // B- &str (String slice)
    // They are a reference to a string stored in memory, and they are immutable.
    // Good for memory efficiency, and are fixed, and are stored on the stack, hence cannot grow

    let string: String = String::from("Hello World");
    let slice: &str = &string[0..5];
    println!("Slice Value: {}", slice);
}
