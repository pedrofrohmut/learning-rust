#![allow(dead_code)]
#![allow(unused_variables)]

use core::panic;
use std::error::Error;
use std::fs::{File, self};
use std::io::ErrorKind;
use std::io;
use std::io::Read;

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

fn read_username_from_file() -> Result<String, io::Error>
{
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut username = String::new();

    let read_result = match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(error) => Err(error),
    };
    return read_result;
}

fn read_username_from_file_simplified() -> Result<String, io::Error>
{
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    return Ok(username);
}

fn read_username_from_file_chaining() -> Result<String, io::Error>
{
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    return Ok(username);
}

fn read_username_from_file_oneline() -> Result<String, io::Error>
{
    fs::read_to_string("hello.txt")
}

// @bash > RUST_BACKTRACE=1 cargo run
// Add the return type to main so you can use ? operator
fn main() -> Result<(), Box<dyn Error>>
{
    let file = File::open("hello.txt")?;
    Ok(())
}
