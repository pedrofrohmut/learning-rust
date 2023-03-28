#![allow(dead_code)]
#![allow(unused_variables)]

use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil
}

use crate::List::{Cons, Nil};

fn main()
{
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("Count after creating a: {}", Rc::strong_count(&a));

    { // B Scope
        let b = Cons(3, Rc::clone(&a));
        println!("Count after creating b: {}", Rc::strong_count(&a));

        { // C Scope
            let c = Cons(4, Rc::clone(&a));
            println!("Count after creating c: {}", Rc::strong_count(&a));
        }

        println!("Count after c goes out of scope: {}", Rc::strong_count(&a));
    }

    println!("Count after b goes out of scope: {}", Rc::strong_count(&a));
}
