#![allow(unused)]

use std::io;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use rand::Rng;

use std::thread;
use std::time::Duration;

fn main()
{
    pub struct Bank {
        balance: f32
    }

    fn withdraw(the_bank: &mut Bank, amount: f32) {
        the_bank.balance -= amount;
    }

    let mut bank = Bank { balance: 100.0 };
    withdraw(&mut bank, 5.0);
    println!("Balance: {}", bank.balance);

    fn customer(the_bank: &mut Bank) {
        withdraw(the_bank, 5.0);
    }

    thread::spawn(|| {
        customer(&mut bank);
    }).join().unwrap();
}
