#![allow(dead_code)]
#![allow(unused_variables)]

struct User {
    username: String,
    email: String,
    signed_in_count: u64,
    active: bool,
}

// Tuple structs are useful when you want a tuple to have a name
// and you want to diferentiate one tuple from another and also
// when you dont care about the field names.

// Same tuple different names
struct Color(u8, u8, u8);
struct Point(u8, u8, u8);

// Use Tuple Structs where you can omit field names to reduce verbosit
struct RgbColor {
    red: u8,
    green: u8,
    blue: u8,
}
struct RGB(u8, u8, u8);

fn main()
{
    // Default create a struct with field names. Order can be different of the
    // order declared in the struct
    let mut user1 = User {
        email: String::from("john@doe.com"),
        username: String::from("john_doe"),
        active: true,
        signed_in_count: 1,
    };

    // Fields can be accessed by dot notation
    let name = user1.username;
    user1.username = String::from("johnny");

    // Create a struct with a builder method
    let user2 = build_user(String::from("jane@smith.com"), String::from("jane_smith"));

    // Create another struct using fields of an existing struct
    let user3 = User {
        email: String::from("james@mail.com"),
        username: String::from("james123"),
        ..user2
    };
}

fn build_user(email: String, username: String) -> User
{
    User { email, username, active: true, signed_in_count: 1 }
}
