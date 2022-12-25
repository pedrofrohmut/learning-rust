#![allow(unused)]

use std::io;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use rand::Rng;

trait Shape {
    fn new(length: f32, width: f32) -> Self;
    fn area(&self) -> f32;
}

struct Rectangle { length:f32, width: f32 }

impl Shape for Rectangle {
    fn new(length: f32, width: f32) -> Rectangle {
        return Rectangle { length, width };
    }
    fn area(&self) -> f32 {
        return self.length * self.width;
    }
}

const PI: f32 = 3.141592;

struct Circle { length:f32, width: f32 }

impl Shape for Circle {
    fn new(length: f32, width: f32) -> Circle {
        return Circle { length, width };
    }
    fn area(&self) -> f32 {
        let r = (self.length / 2.0);
        return PI * r.powf(2.0);
    }
}


fn main() {
    let rec1 = Rectangle::new(10.0, 10.0);
    println!("Rectangle area: {}", rec1.area());

    let cir1 = Circle::new(10.0, 10.0);
    println!("Circle area: {}", cir1.area());
}
