#![allow(unused_variables)]

fn main()
{
    /*
       The Rules of References

       1. At any given time you can have either one mutable reference or any
       number of immutable references

       2. References must always be valid
    */

    { // Here you borrow s1 to get len so you dont have to return it back
        let s1 = String::from("hello");
        let len = calculate_length(&s1); // It does not move, only borrows
        println!("S1 = {} , Len = {}", s1, len);
    }

    { // Mutable borrow
        let mut s1 = String::from("Hello");
        change_str(&mut s1);
    }

    {
        let mut s = String::from("Hello");
        let s1 = &mut s;
        let s2 = &mut s; // You cannot more than one mutable reference
        println!("S1 = {}, S2 = {}", s1, s2);
    }

    {
        let mut s = String::from("Hello");
        let s1 = &s;
        let s2 = &s;
        println!("S1 = {}, S2 = {}", s1, s2);

        // At this point you can pass a mutable reference because it will not
        // affect any other variables at this point
        let s3 = &mut s;
        println!("S3 = {}", s3);
    }

    {
        // Rust prevents (memory unsafe) dangling points. The compiler will not
        // allow pointers that points to nowhere to be passed around

        // You cannot return a reference to s because when this scoped end s will
        // be deallocated. And the reference will point to nothing.
        let ref_to_nothing = dangle();
    }
}

fn calculate_length(s: &String) -> usize { s.len() }

fn change_str(s: &mut String) { s.push_str(", World!") }

fn dangle() -> &String
{
    let s = String::from("hello");
    return &s;
}
