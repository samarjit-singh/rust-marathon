enum Shape {
   Rectangle(f64, f64), // width, height
   Circle(f64),         // radius
}

fn main() {
    let rect = Shape::Rectangle(1.0, 2.0);
    let area_rect = calculate_area(rect);
    println!("Area of Rectangle: {}", area_rect);

    let circle = Shape::Circle(2.0);
    let area_circle = calculate_area(circle);
    println!("Area of Circle: {}", area_circle); 
}

fn calculate_area(shape: Shape) -> f64 {
    let area = match shape {// pattern matching
        Shape::Rectangle(a, b) => a * b,
        Shape::Circle(r) => 3.14 * r * r,
    };
    return area;
}
