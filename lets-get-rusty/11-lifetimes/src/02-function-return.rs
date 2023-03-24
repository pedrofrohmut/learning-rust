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
    }

    // Here the result is valid because str2 is not part of the lifetime anymore
    println!("The longest string is {}", result);
}


// The longest2 is valid because now str2 is no longer of 'a lifetime and only
// str1 is checked for validation
fn longest2<'a>(str1: &'a str, str2: &str) -> &'a str { str1 }

fn longest3<'a>(str1: &'a str, str2: &str) -> &'a str
{
    let result = String::from("Hello, World!");
    return result.as_str();
}

// When you return a reference from a function it must be one of the arguments
// passed to a function (outside scope) a not a reference to something created
// inside the function scope

// All the references created inside the function will be deallocated when it
// finishes executing. So they will become dangling pointers
fn longest4<'a>(str1: &'a str, str2: &str) -> &'a str
{
    let result = String::from("Hello, World!");
    return result.as_str();
}

// Here you can return because you are transfering ownership and not just passing
// a reference to a resource
fn longest5<'a>(str1: &'a str, str2: &str) -> String
{
    let result = String::from("foo bar");
    return result;
}
