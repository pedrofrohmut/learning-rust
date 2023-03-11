fn main() {
    // let - Rust is immutable by default. You need to add the 'mut' keyword
    // when you want to change the value of a variable
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // const - constants cannot be mutable with 'mut' and you need to declare the type
    const MEMBERS_COUNT: u32 = 123;

    // Shadowing - allows you, using the 'let' keyword, to shadow the previous value
    // of a variable without making it mutable.
    // Shadowing is very usefull when you just parse a value and dont want to create
    // a new name for a variable too
    let y = 5;
    println!("The value of 'y' is {}", y);
    let y = y + 8;
    println!("The value of 'y' is {}", y);
}
