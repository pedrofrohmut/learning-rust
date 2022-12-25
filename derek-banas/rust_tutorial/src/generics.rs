#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

use std::ops::Add;

fn sum<T:Add<Output = T>>(x1: T, x2: T) -> T {
    return x1 + x2;
}

fn main() {
    println!("5 + 4 = {}", sum(5, 4));
    println!("5.2 + 4.6 = {}", sum(5.2, 4.6));
}
