#![allow(unused)]

use std::io;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use rand::Rng;

use std::thread;
use std::time::Duration;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};

fn main()
{
    pub struct Bank {
        balance: f32
    }

    fn withdraw(the_bank: &Arc<Mutex<Bank>>, amount: f32) {
        let mut bank_ref = the_bank.lock().unwrap();
        if bank_ref.balance < 5.00 {
            println!("Current balance: {}. Withdraw an smaller amount", bank_ref.balance);
        } else {
            bank_ref.balance -= amount;
            println!("Customer withdrew {}. The current balance is {}", amount, bank_ref.balance);
        }
    }

    fn customer(the_bank: Arc<Mutex<Bank>>) {
        withdraw(&the_bank, 5.0);
    }

    let bank = Arc::new(Mutex::new(Bank { balance: 200.0 }));
    let handles = (0..10).map(|_| {
        let bank_ref = bank.clone();
        thread::spawn(|| {
            customer(bank_ref)
        })
    });

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Total: {}", bank.lock().unwrap().balance);
}
