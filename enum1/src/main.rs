enum Shape {
    Square { s: f64 },
    Circle { r: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Square { s } => s * s,
            Shape::Circle { r } => r* 3.14,
        }
    }
}

fn main() {
    let s = Shape::Square { s: 10.0 };
    println!("area is {}", s.area());
}