#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.area() >= rectangle.area()
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 50,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect3 = Rectangle {
        width: 70,
        height: 70,
    };

    let square: Rectangle = Rectangle::square(3);

    println!("This is a square: {:#?}", square);
    println!("The area of the rectangle is {} square pixels.", rect1.area());
    println!("rect2 can fit inside of rect1: {}", rect1.can_hold(&rect2));
    println!("rect3 can fit inside of rect1: {}", rect1.can_hold(&rect3));
}