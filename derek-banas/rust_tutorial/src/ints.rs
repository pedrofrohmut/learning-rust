#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    // Unsigned int: u8, u16, u32, u64, u128, usize
    // Signed   int: i8, i16, i32, i64, i128, isize
    println!("Max for u32: {}"  , u32::MAX);
    println!("Max for u64: {}"  , u64::MAX);
    println!("Max for usize: {}", usize::MAX);
    println!("ETC...");

    let is_true = true;
    let my_grade = 'A';
}

