#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    let arr1 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    // By positon
    println!("1st = {}", arr1[0]);
    // Get length
    println!("length = {}", arr1.len());

    // Loop through
    let mut i = 0;
    loop {
        if arr1[i] % 2 == 0 {
            i += 1;
            continue;
        }
        if arr1[i] == 9 {
            break;
        }
        println!("Loop Val: {}", arr1[i]);
        i += 1;
    }

    // While
    let mut i2 = 0;
    while i2 < arr1.len() {
        println!("While Val: {}", arr1[i2]);
        i2 += 1;
    }

    // For
    let mut i3 = 0;
    for val in arr1.iter() {
        println!("For Val: {}", val);
    }
}
