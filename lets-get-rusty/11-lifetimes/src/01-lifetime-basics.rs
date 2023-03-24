#![allow(unused_variables)]
#![allow(dead_code)]

fn main()
{
    let result: &str;

    let str1 = String::from("abcd");
    {
        let str2 = String::from("xyz");
        // result will have the smallest lifetime of the function arguments.
        // So will be equal the the lifetime of str2 in this example.
        result = longest2(str1.as_str(), str2.as_str());

        // Here result lifetime is valid because str1 and str2 are both valid
        println!("The longest string is {}", result);
    }

    // Str2 does not live long enough
    // Here result is no longer valid because str1 is valid but str2 is not
    println!("The longest string is {}", result);
}

// &i32          - a reference
// &'a i32       - a reference with a explicit lifetime
// &'a mut i32   - a mutable reference with an explicit lifetime

// The lifetime annotation creates a relationship between of the members under
// the same value (ex: 'a).

// The lifetime of the function return will be the same as the smallest lifetime
// of the arguments
fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str
{
    if str1.len() > str2.len() { str1 } else { str2 }
}
