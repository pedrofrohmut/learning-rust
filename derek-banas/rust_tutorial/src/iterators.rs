#![allow(unused)]

use std::io;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let mut arr1 = [1, 2, 3, 4];
    for n in arr1.iter() {
        println!("{}", n);
    }

    let mut iter1 = arr1.iter();
    println!("1st {:?}", iter1.next());
}
