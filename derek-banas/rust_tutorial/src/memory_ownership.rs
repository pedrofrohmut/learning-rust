#![allow(unused)]

use std::io;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use rand::Rng;

fn print_str(x: String) {
    println!("Hello {} 2", x);
}

fn print_return_str(x: String) -> String {
    println!("Hello {} 3", x);
    // return x;
    x
}

fn change_str(name: &mut String) {
    name.push_str(" is happy");
    println!("Message: {}", name);
}

fn main() {
    let str1 = String::from("World");
    println!("Hello {} 1", str1);

    print_str(str1.clone());

    let str3 = print_return_str(str1);
    println!("Hello {} 4", str3);

    // If you just assign the str1 to str2 than str1 will no longer exist
    // Then to both exist you need to clone it
    let mut str2 = String::from("Bob");
    change_str(&mut str2);
}
