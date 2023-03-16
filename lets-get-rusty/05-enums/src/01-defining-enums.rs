#![allow(dead_code)]
#![allow(unused_variables)]

enum IpAddressKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddress {
    kind: IpAddressKind,
    address: String,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Text(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn some_function()
    {
        println!("Function called");
    }
}

// enum Option<T> {
//     Some(T),
//     None,
// }

fn main()
{
    let four = IpAddressKind::V4;
    let six = IpAddressKind::V6;
    let localhost = IpAddressKind::V4(127, 0, 0, 1);

    let some_number = Some(5);
    let some_string = Some("A string");
    let absent_number: Option<i32> = None;

    let x: i32 = 5;
    let y: Option<i32> = Some(5);

    let sum = x + y.unwrap_or(0);
}

fn route(ip_kind: IpAddressKind) {}
