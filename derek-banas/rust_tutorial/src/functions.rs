#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn say_hello() {
    println!("Hello");
}

fn print_sum(x: i32, y: i32) {
    println!("{} + {} = {}", x, y, x + y);
}

fn sum(a: i32, b: i32) -> i32 {
    return a + b;
}

fn my_fun(arg1: i32) -> (i32, i32) {
    return (arg1 + 1, arg1 + 2);
}

fn reduce_sum(nums: &[i32]) -> i32 {
    let mut sum = 0;
    for &val in nums.iter() {
        sum += &val;
    }
    return sum;
}

fn main() {
    say_hello();

    print_sum(35, 34);

    println!("{}", sum(400, 20));

    let result: (i32, i32) = my_fun(69);
    println!("{}, {}", result.0, result.1);

    let (val1, val2) = my_fun(69);
    println!("{}, {}", val1, val2);

    let numbers = vec![1, 2, 3, 4, 5];
    println!("List sum: {}", reduce_sum(&numbers));
}
