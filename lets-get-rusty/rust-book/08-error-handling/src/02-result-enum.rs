#![allow(dead_code)]
#![allow(unused_variables)]

use core::panic;
use std::fs::File;
use std::io::ErrorKind;

enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn error_panic()
{
    let file = match File::open("hello.txt") {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file {:?}", error),
    };
}

fn error_kind_not_found()
{
    let file = match File::open("hello.txt") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file_created) => file_created,
                Err(error) => panic!("Preblem creating a file: {:?}", error),
            },
            other_error => panic!("Some other of error: {:?}", other_error),
        },
    };
}

fn error_unwrap_or_else()
{
    let file = File::open("hello.txt").unwrap_or_else(|err| {
        if err.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|err| {
                panic!("Problem creating a file: {:?}", err);
            })
        } else {
            panic!("Problem opening the file: {:?}", err);
        }
    });
}

fn error_unwrap() -> File
{
    File::open("Hello.txt").unwrap()
}

fn error_expect() -> File
{
    File::open("hello.txt").expect("Failed to open the file")
}

// @bash > RUST_BACKTRACE=1 cargo run
fn main()
{
    println!("Hello, error!");
}
