
#![allow(unused)]

use std::io;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use rand::Rng;

mod restaurant;
use crate::restaurant::order_food;

fn main() {
    // Creates: Modules that produce a library or executable
    // Modules: Organize and handle privacy
    // Packages: Build, test and share creates
    // Paths: A way of naming an item such as a structs or functions

    order_food();
}
