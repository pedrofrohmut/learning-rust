#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    let n1: u32 = 5;
    let n2: u32 = 4;
    println!("5 + 4 = {}", n1 + n2);
    let n3 = n1 + n2;
    println!("n3 = {}", n3);

    let rn1 = rand::thread_rng().gen_range(1..101);
    println!("Random: {}", rn1);
}
