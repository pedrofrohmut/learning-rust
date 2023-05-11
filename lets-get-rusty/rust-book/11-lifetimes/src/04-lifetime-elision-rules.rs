#![allow(unused_variables)]
#![allow(dead_code)]

fn main()
{
}

/*
    Lifetime Elision Rules

    1. Each parameter that is a reference gets its own lifetime parameter

    2. If there is exactly one input lifetime parameter, that lifetime is
    assigned to all output lifetime parameters

    3. If there are multiple input lifetime parameters, but one of them is
    &self or &mut self the lifetime of self is assigned to all output lifetime
    parametes
*/

// The compiler generates: fn first_word<'a>(input_str: &'a str) -> &'a str

// By following the rules 1 and 2 the compiler is able to automatically assign
// the lifetime for the input and output
fn first_word(input_str: &str) -> &str
{
    let bytes = input_str.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &input_str[0..i];
        }
    }

    &input_str[..]
}
