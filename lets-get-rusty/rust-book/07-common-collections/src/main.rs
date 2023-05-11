#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;

fn main()
{
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::<String, i32>::new();

    scores.insert(blue, 10);
    scores.insert(yellow, 50);

    let team_name = String::from("Blue");
    let team_score = scores.get(&team_name);

    for (key, value) in scores {
        println!("{}: {}", key, value);
    }

    let mut scores = HashMap::new();

    // Overrides the value with the key "Blue"
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 20);

    // If there is not key/value pair for "Yellow" insert otherwise do nothing
    scores.entry(String::from("Yellow")).or_insert(30);
    scores.entry(String::from("Yellow")).or_insert(40);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // If the key does not exist in the hashmap inserts it with the default
        // value else do nothing and returns a mutable reference to it
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
