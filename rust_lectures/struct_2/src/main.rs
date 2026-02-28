fn main() {
    let student1 = Student {
        name: String::from("John"),
        age: 20,
        grade: 15.5,
    };
    let student2 = Student {
        name: String::from("Shien"),
        age: 22,
        grade: 90.0,
    };

    println!("Name: {}", student1.name);
    println!("Age: {}", student1.age);
    println!("Grade: {}", student1.grade);
    println!("Has student1 passed: {}", student1.is_passed());
    println!("Has student2 passed: {}", student2.is_passed());
}
struct Student {
    name: String,
    age: u32,
    grade: f32,
}

impl Student {
    fn is_passed(&self) -> bool {
        self.grade >= 50.0
    }
}
