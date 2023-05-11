#![allow(dead_code)]
#![allow(unused_variables)]

fn main()
{
    let arr = [1, 2, 3];

    let mut vec = Vec::<i32>::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    let mut v2 = vec![1, 2, 3, 4, 5];

    let third = &v2[2];
    // You cannot modify the vector and use any immutable reference after doing it
    // v2.push(6);
    println!("Third with index (&v2[2]): {}", third);

    match v2.get(2) {
        Some(third) => println!("The third element with get (v2.get(2)): {}", third),
        None        => println!("There is no third element"),
    }

    match v2.get(20) {
        Some(twentyth) => println!("The twentyth element is: {}", twentyth),
        None           => println!("There is no twentyth element"),
    }

    let val20: i32 = match v2.get(20) {
        Some(x) => *x,
        None => 0
    };
    println!("Value 20: {}", val20);

    // Print taking immutable references
    for i in &v2 { println!("{}", i); }

    // Modifying the vector with mutable references
    for i in &mut v2 { *i += 50; }
    for i in &v2 { println!("{}", i); }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(2.17),
        SpreadsheetCell::Text(String::from("Hello, World!")),
    ];

    match &row[1] {
        SpreadsheetCell::Int(x) => println!("{}", x),
        _                       => println!("Not an integer"),
    }
}
