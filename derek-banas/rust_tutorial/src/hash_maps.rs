#![allow(unused)]

use std::io;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use rand::Rng;

use std::collections::HashMap;

fn main() {
    let mut heroes = HashMap::new();
    heroes.insert("Superman", "Clark Kent");
    heroes.insert("Batman", "Bruce Wayne");
    heroes.insert("The Flash", "Barry Allen");

    for (key, value) in heroes.iter() {
        println!("{} = {}", key, value);
    }

    println!("Length: {}", heroes.len());

    if heroes.contains_key("Batman") {
        let the_batman = heroes.get(&"Batman");
        match the_batman {
            Some(x) => println!("Batman is a hero"),
            None    => println!("Batman is not a hero")
        }
    }
}
