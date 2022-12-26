#![allow(unused)]

use std::io;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    //# Error1: just panic! macro
    panic!("Terrible error");

    //# Error2: try accessing an index bigger than the array
    let my_arr = [1, 2];
    println!("{}", my_arr[10]);

    //# Error3: Try opening a file
    let path = "lines.txt";
    let output = File::create(path);
    // Result has 2 varients Ok and Err Where T represents the data typeof the
    // value returns and E the typeof error
    //   enum Result<T, E> {
    //     Ok(T),
    //     Err(T),
    //   }
    let mut output = match output {
        Ok(file)   => file,
        Err(error) => panic!("Problem creating file {:?}", error),
    };

    // 'expect' is a handly function to show error messages when something triggers an error
    write!(output, "Just some \nRandom words").expect("Failed to write to file");

    // 'unwrap' ignores the result step and just gives us the file
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    for line in buffered.lines() {
        println!("{}", line.unwrap());
    }

    // Catch specific errors
    let output2 = File::create("rand.txt");
    let output2 = match output2 {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("rand.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Can't create file: {:?}", err),
            },
            _other_error => panic!("Problem opening the file: {:?}", err),
        }
    };
}
