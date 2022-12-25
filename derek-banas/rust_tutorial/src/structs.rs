#![allow(unused)]

use std::io;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use rand::Rng;

struct Customer {
    name: String,
    address: String,
    balance: f32,
}

fn print_customer(customer: Customer) {
    println!("Name: {}", customer.name);
    println!("Address: {}", customer.address);
    println!("Balance: {}", customer.balance);
}

fn main() {
    let mut bob = Customer {
        name: String::from("Bob Smith"),
        address: String::from("666, Main st"),
        balance: 420.69
    };
    bob.address = String::from("123, Other st");
    print_customer(bob);
}
