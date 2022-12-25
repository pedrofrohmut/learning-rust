#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    let age = 38;
    // Regular if/else if/else
    if (age >= 1) && (age <= 18) {
        println!("Important birthday");
    } else if (age == 21) || (age == 50) {
        println!("Important birthday");
    } else if (age >= 65) {
        println!("Important birthday");
    } else {
        println!("Sorry, Just a regular birthday");
    }

    let age2 = 47;
    // If as an expression
    let can_vote = if age2 >= 18 {
        true
    } else {
        false
    };
    println!("Can vote? {}", can_vote);

    let age3 = 8;
    // 1..=18 is the range from 1 to 18 (including 18)
    match age3 {
        1..=18        => println!("Important birthday"),
        21 | 50       => println!("Important birthday"),
        65..=i32::MAX => println!("Important birthday"),
        _             => println!("Sorry, Just a regular birthday")
    };

    let age4 = 18;
    let voting_age = 18;
    match age4.cmp(&voting_age) {
        Ordering::Less    => println!("Can't vote"),
        Ordering::Greater => println!("Can vote"),
        Ordering::Equal   => println!("You just gained the right to vote")
    };
}
