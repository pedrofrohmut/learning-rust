#![allow(dead_code)]
#![allow(unused_variables)]

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self
    {
        Self { x, y }
    }
}

impl<T> Pair<T> where T: Display + PartialOrd {
    fn cmd_display(&self)
    {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// Blanket implementations: you implement ToString trait form a generic Type
// that implements Display
impl<T> ToString for T where T: Display {}

impl<T: Display> ToString for T {}

impl<T> Pair<T> where T: Display {}

// Impl of ToString trait for Pair<T> struct
impl<T> ToString for Pair<T> where T: Display {}

fn main()
{
}
