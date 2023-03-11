#![allow(unused_variables)]

fn main()
{
    // Control flow
    let number = 5;

    // Statements if
    // In rust the conditions MUST BE A BOOLEAN (there are no truthy and falsy)
    if number > 10 {
        println!("First condition");
    } else if number > 22 {
        println!("Second condition");
    } else {
        println!("Else condition");
    }

    // One Line if expression (Rust version of ternary operator)
    let result = if number > 42 { "You got it!" } else { "Sorry. You failed." };
    println!("Result is: {}", result);

    // Infinite loop - You need to manually provide an escape with break, return, etc
    loop {
        println!("Print me!");
        break
    }

    // Loop with a counter
    let mut counter = 0;
    loop {
        if counter > 5 { break; }
        println!("Couter: {}", counter);
        counter += 1;
    }

    // Loop that returns a value
    let mut nums = 0;
    let score = loop {
        if nums > 12 {
            break nums; // Break out of the loop and return nums
        }
        nums += 5;
    };
    println!("Score: {}", score);

    // While loop
    let mut i = 0;
    while i < 10 {
        println!("I = {}", i);
        i += 1;
    }

    // Do while? there is none. Use loop + break
    let mut j = 0;
    loop {
        println!("J = {}", j);
        j += 1;
        if j > 10 { break; }
    }

    // For each
    let arr = [42, 13, 666, 144];
    for x in arr {
        println!("x = {}", x);
    }

    // For each with ranges
    // ranges are end exclusive use equal sign to include the end value
    // (1..3) => 1, 2 and (1..=3) is 1, 2, 3 and (1..=3).rev() is 3, 2, 1
    // rev - Reverse
    for n in (1..=5).rev() {
        println!("n = {}", n);
    }
}
