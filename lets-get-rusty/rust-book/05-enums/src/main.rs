#![allow(dead_code)]
#![allow(unused_variables)]

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8
{
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32>
{
    // Matching expressions are exaustive. You have to match all possible cases
    // or it will be considered an error
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main()
{
    value_in_cents(Coin::Quarter(UsState::Alaska));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // Use underscore as last to match everything else
    let some_value = Some(3);
    match some_value {
        Some(3) => println!("Three"),
        _ => (),
    }

    // If let syntax. Works the same as the above
    if let Some(3) = some_value {
        println!("Three")
    };
}
