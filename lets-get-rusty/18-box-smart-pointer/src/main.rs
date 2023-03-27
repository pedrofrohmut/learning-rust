enum List {
    Cons(i32, Box<List>),
    Nil
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

use List::{Cons, Nil};

fn main()
{
    // Stores 5 on the heap
    let x = Box::new(5);
    println!("x = {}", x);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
