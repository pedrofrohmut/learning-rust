#![allow(unused)]

use std::io;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use rand::Rng;

// Closures
// let my_func = |param| -> return_type { BODY }
fn main()
{
    // Basic
    let can_vote = |age: u32| -> bool {
        age >= 18
    };
    println!("{}", can_vote(24));
    println!("{}", can_vote(14));

    // Closures have access to outside variables
    let mut samp1 = 5;
    let print_var = || println!("samp1 = {}", samp1);
    print_var();
    samp1 = 10;
    let mut change_var = || samp1 += 1;
    change_var();
    println!("samp1 = {}", samp1);
    samp1 = 7;
    println!("samp1 = {}", samp1);

    //
    fn use_func<T>(a: i32, b: i32, func: T) -> i32 where T: Fn(i32, i32) -> i32 {
        func(a, b)
    }
    let sum = |a, b| a + b;
    println!("5 + 4 = {}", use_func(5, 4, sum));
    let prod = |a, b| a * b;
    println!("5 * 4 = {}", use_func(5, 4, prod));
}
