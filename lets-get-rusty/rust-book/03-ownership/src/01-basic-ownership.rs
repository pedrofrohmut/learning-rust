#![allow(unused_variables)]

fn main()
{
    /*
       Ownership Rules

       1. Each variable in rust has ownership of its value
       2. There can be only one owner at a time
       3. When then owner goes out of scope. The resource will be freed
       */

    // Scope of s: in the end of this scope the String "Hello" will be deallocated
    // unless the ownership is transfered out of the scope.
    {
        // "Hello" lives as long as it owner lives
        let s = String::from("Hello");
    }
    // The scope end 's' is removed from the stack and "Hello" deallocated from
    // the heap

    // Simple values are just copied by default
    let x = 5;
    let y = x; // Copy

    // s1 has: 1 pointer to its value in the heap / its length / its capacity
    let s1 = String::from("Foo");
    let s2 = s1; // Moved: Ownership is transfered from s1 to s2
    // Now s1 does not own the "Foo" anymore and does not have access to it
    println!("S1 = {}", s1);

    let s3 = String::from("Hello");
    takes_ownership(s3); // Moved: Ownership transfered to the function
    // Now s3 is not valid anymore
    println!("S3 = {}", s3);

    let x = 5;
    copy_value(x);
    // Now x is still valid because integers are copied by default
    println!("X = {}", x);

    let s4 = gives_ownership();

    { // The function returns ownership of s2 so the value is not deallocated
        let s1 = gives_ownership();
        let s2 = String::from("Hello");
        let s3 = takes_and_gives_back(s2); // Return ownership of s2
        // Here s2 is not valid because s3 owns s2 value
        println!("S1 = {}, S3 = {}", s1, s3);
    }

    { // Here you pass s1 to get len but need to return ownership to not lost
        let s1 = String::from("hello");
        let (len, s2) = calculate_length(s1); // Returns ownership of s2
        println!("S2 = {}, Len = {}", s2, len);
    }
}

fn takes_ownership(some_string: String) { println!("{}", some_string) }

fn copy_value(i: i32) { println!("{}", i) }

fn gives_ownership() -> String { String::from("Hello") }

fn takes_and_gives_back(s: String) -> String { s }

fn calculate_length(s: String) -> (usize, String) { (s.len(), s) }
