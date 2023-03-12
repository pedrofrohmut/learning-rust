#![allow(unused_variables)]

fn main()
{
    // String slices - &str

    {
        let mut s = String::from("Hello World");
        // Slice with 'Hello' if start at the begin you can omit left value (0 here)
        let hello = &s[..5];
        // Slice with 'World' if goes untill last index you can omit right value (11 here == s.len())
        let world = &s[6..];

        let first_word = first_word(&s);
        s.clear();
        println!("First World: {}", first_word);
    }

    {
        // String literals are string slices
        let s2 = "Hello World";
        let word = first_word2(s2);
    }

    let arr = [1, 2, 3, 4, 5, 6];
    let slice1 = &arr[..3]; // Slice of integers
}

fn first_word(s: &String) -> &str
{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { return &s[..i]; }
    }
    return &s[..]; // Slice of entire string: Omit both right and left value
}

fn first_word2(s: &str) -> &str
{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { return &s[..i]; }
    }
    return &s[..]; // Slice of entire string: Omit both right and left value
}
