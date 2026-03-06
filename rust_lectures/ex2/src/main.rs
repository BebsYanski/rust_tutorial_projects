fn main() {
    let circle1 = Shape::Circle(10.0);
    let rectangle1 = Shape::Rectangle(10.0, 20.0);

    println!("Area of circle1: {}", area(circle1));
    println!("Area of rectangle1: {}", area(rectangle1));
}

enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
}

fn area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => {
            let area = std::f64::consts::PI * radius * radius;
            area
        }
        Shape::Rectangle(width, height) => {
            let area = width * height;
            area
        }
    }
}
