struct Rect {
    width: i32,
    height: i32,
}

impl Rect {
    fn area(&self) -> i32 {
        self.width * self.height // implicite return
    }

    fn perimeter(&self) -> i32 {
        2 * (self.width + self.height)
    }
}

fn main() {
    let rect1 = Rect {
        width: 10,
        height: 10
    };

    println!("area is {}", rect1.area());
    println!("perimeter is {}", rect1.perimeter());
}