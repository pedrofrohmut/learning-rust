use std::{io::stdin, cmp::Ordering};
use rand::Rng;
use colored::*;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number {}", secret_number);
    println!("Guess the Number Game");

    loop {
        println!("Please, input your guess");

        let mut guess = String::new();
        stdin().read_line(&mut guess).expect("Failed to read line");
        // println!("You guessed: {}", guess);

        let guess: i32 = match guess.trim().parse() {
            Ok(n)  => n,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("{}", "Too small".red()),
            Ordering::Greater => println!("{}", "Too big".red()),
            Ordering::Equal   => {
                println!("{}", "You guessed!".green());
                break;
            }
        }
    }
}
