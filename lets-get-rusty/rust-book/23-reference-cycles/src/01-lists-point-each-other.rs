#![allow(dead_code)]
#![allow(unused_variables)]

use std::{cell::RefCell, rc::Rc};

use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>>
    {
        match self {
            Cons(_, item) => Some(item),
            Nil => None
        }
    }
}

// Box: Single owner -> mutable or immutable borrow compile time
// Rc: Multiple owners -> only immutable borrows
// RefCell: Single owner -> mutable borrows at runtime
// Multiple and mutable?
//     You need a Rc of a RefCell
fn main()
{
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("A initial Rc count: {}", Rc::strong_count(&a));
    println!("A next item: {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("A Rc count after b creation: {}", Rc::strong_count(&a));
    println!("B initial count: {}", Rc::strong_count(&b));
    println!("B next item: {:?}", b.tail());

    if let Some(link) = a.tail() {
        // Here you clone List B in the tail of List A
        // But List B tail is List A with results in a cycling reference
        // A: (5, -> B) and B: (10, -> A)
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("B Rc count after changing A: {}", Rc::strong_count(&b));
    println!("A Rc count after changing A: {}", Rc::strong_count(&a));

    // This next line generated an stack overflow
    println!("A next item: {:?}", a.tail());
}
