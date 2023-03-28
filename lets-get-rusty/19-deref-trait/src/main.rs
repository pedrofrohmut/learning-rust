#![allow(dead_code)]

use std::ops::Deref;

// Defining our own Smart Pointer
struct MyBox<T>(T);// Tuple struct

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> { MyBox(x) }
}

// With Deref Trait you can use deref operator '*' with anything that implements
// this trait no only references
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target { &self.0 }
}

// Implicit Deref Coercion
fn hello(name: &str) { println!("Hello, {}!", name); }

fn main()
{
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    // Rust calls: *(y.deref()) when it sees the *y
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));

    // &MyBox<String> -> &String -> &str
    // Rust Compiler auto dereference to &str, so the code is easier to read
    hello(&m);

    // Without auto deref you need to write code like this
    hello(&(*m)[..]);
}
