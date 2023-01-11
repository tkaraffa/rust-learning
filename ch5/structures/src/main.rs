#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        (self.width > rectangle.width) && (self.height > rectangle.height)
    }

    fn square(side: u32) -> Self {
        Self {
            width: side,
            height: side,
        }
    }
}

fn main() {
    let square = Rectangle::square(10);

    println!("{}", square.area());
}
