fn main() {
    println!("ToDo List app");
    let task1 = task::new(String::from("Task 1"));
    println!("{}", task1.name);
    println!("{}", task1.completed);
}

struct task {
    name: String,
    completed: bool,
}

impl task {
    fn new(name: String) -> task {
        task {
            name,
            completed: false,
        }
    }
}
