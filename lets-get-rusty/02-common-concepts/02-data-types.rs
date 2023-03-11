#![allow(unused_variables)]

fn main() {
    // Integer - default is signed 32bits (i32)
    let decimal = 98_000;      // Decimal
    let hex     = 0xff;        // Hexa Decimal
    let octal   = 0o77;        // Octal
    let binary  = 0b1111_0100; // Binary
    let byte    = b'A';        // Byte (u8 only)

    // Rust compiler will not allow integer overflow
    let short: u8 = 255;

    // Float - default is 64bits double precision (f64)
    let float1      = 2.0;
    let float2: f32 = 3.0; // You can add the type when you want more or less precision

    // Addition
    let sum = 5 + 10;
    // Subtraction
    let diff = 9.0 - 3.0;
    // Multiplication
    let product = 4 * 20;
    // Division
    let quotient = 50.0 / 2.0;
    // Remainder
    let remainder = 5 % 2;

    // Booleans
    let bool1  = true;
    let bool2 = false;

    // Character
    let char1 = 'a';

    // Compond types -----------------------------------------------------------

    // Tuples
    let tuple1 = ("Let's Get Rusty!", 100_000);
    // Destructuring
    let (channel, sub_count) = tuple1;
    // Get values with dot notation
    let sup_count = tuple1.1; // position 1 of the tuple

    // Arrays in rust have fixed lenght
    let error_codes = [200, 404, 500];
    // Access - Access the position 1 of the array
    let not_found = error_codes[1];
    // Initialize - Create an array filled with zeros with size 8
    let byte = [0; 8];

    // You get run time error index out of bounds
    let out_of_bounds = error_codes[666];
}
