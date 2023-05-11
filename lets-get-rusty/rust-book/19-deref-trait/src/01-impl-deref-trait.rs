#![allow(dead_code)]

use std::ops::Deref;

// Tuple struct
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T>
    {
        MyBox(x)
    }
}

// With Deref Trait you can use deref operator '*' with anything that implements
// this trait no only references
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target
    {
        &self.0
    }
}

fn main()
{
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    // Rust calls: *(y.deref()) when it sees the *y
    assert_eq!(5, *y);
}
