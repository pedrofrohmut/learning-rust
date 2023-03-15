#![allow(dead_code)]
#![allow(unused_variables)]

// Add derive(Debug) so you can use {:?} on println!
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// With impl Rectangle you make functions into methods
// Methods are functions associated with a struct
impl Rectangle {
    // The first argument of method is always &self (a reference to self - this in JS)
    fn area(&self) -> u32
    {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool
    {
        self.width >= other.width && self.height >= other.height
    }
}

// Associative functions you call the with :: and they are not related the an instance
// but to the struct directly
impl Rectangle { // Rust allows to have more the 1 impl block
    fn square(size: u32) -> Rectangle
    {
        Rectangle { width: size, height: size }
    }
}

fn main()
{
    let rect = Rectangle { width: 30, height: 50 };
    // {:#?} is the same as {:?} but with new lines
    println!("Rect: {:#?}", rect);
    println!("The area of the rectangle is {} square pixels",
             rect.area());

    let rect1 = Rectangle { width: 20, height: 40 };
    println!("\nRect can hold rect1? {}", rect.can_hold(&rect1));

    let rect2 = Rectangle { width: 40, height: 50 };
    println!("Rect can hold rect2? {}", rect.can_hold(&rect2));

    let rect3 = Rectangle::square(25);
    println!("\nRect 3 {:?}", rect3);
}
