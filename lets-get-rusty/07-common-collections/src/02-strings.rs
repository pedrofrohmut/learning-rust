#![allow(dead_code)]
#![allow(unused_variables)]

fn main()
{
    // String is stored as collection of UTF-8 encoded bytes

    // Creating a string
    let s1 = String::new();
    let s2 = "Hello, World!".to_string();
    let s3 = String::from("Hello, World!");

    // Appending strings and characters to a string
    let mut str = String::from("foo");
    str.push_str(" bar");
    str.push('!');
    println!("{}", str);

    // Concatenate strings with + operator
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    // You change ownership of s1 but only borrow s2
    let s3 = s1 + &s2;
    println!("{}", s2);
    println!("{}", s3);

    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    // You dont change ownership neither of s1 nor s2
    let s3 = format!("{}{}", s1, s2);
    println!("{}", s1);
    println!("{}", s2);
    println!("{}", s3);

    let s1 = String::from("Hello World");

    for b in s1.bytes() { println!("{}", b); }

    for c in s1.chars() { println!("{}", c); }
}
