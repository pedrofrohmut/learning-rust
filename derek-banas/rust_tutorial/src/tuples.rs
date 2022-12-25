#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    let my_tuple: (u32, String, f64) = (47, "John".to_string(), 50_000.00);
    // Get tuple value by index
    println!("Name: {}", my_tuple.1);
    // Assign mutiple at the same time
    let(age, name, savings) = my_tuple;
    println!("Age: {}", age);
}
