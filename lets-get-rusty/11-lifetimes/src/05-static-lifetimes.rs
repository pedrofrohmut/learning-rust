#![allow(unused_variables)]
#![allow(dead_code)]

fn main()
{
    // 1. Static lifetime: the lifetime will last as the full duration of the program
    // 2. All string literals have static lifetime
    let s: &'static str = "I have a static lifetime";
}
