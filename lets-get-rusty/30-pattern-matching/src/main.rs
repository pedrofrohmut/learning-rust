#![allow(dead_code)]
#![allow(unused_variables)]

fn main()
{
    // Irrefudable - always match
    let x = 5;

    // Refudable - might not match
    let x: Option<&str> = None;
    if let Some(x) = x {
        println!("{}", x);
    }
}
