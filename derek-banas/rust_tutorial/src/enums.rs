#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday
    }

    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Saturday | Day::Sunday => true,
                _                           => false
            }
        }
    }

    let today: Day = Day::Monday;
    match today {
        Day::Monday    => println!("Everyone hates monday"),
        Day::Tuesday   => println!("Everyone hates monday"),
        Day::Wednesday => println!("Everyone hates monday"),
        Day::Thursday  => println!("Everyone hates monday"),
        Day::Friday    => println!("Everyone hates monday"),
        Day::Saturday  => println!("Everyone hates monday"),
        Day::Sunday    => println!("Everyone hates monday")
    }

    println!("Is today the weekend? {}", today.is_weekend());
}
