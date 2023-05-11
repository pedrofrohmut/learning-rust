#![allow(dead_code)]
#![allow(unused_variables)]

use std::{rc::{Rc, Weak}, cell::RefCell};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>
}

// Box: Single owner -> mutable or immutable borrow compile time
// Rc: Multiple owners -> only immutable borrows
// RefCell: Single owner -> mutable borrows at runtime
// Multiple and mutable?
//     You need a Rc of a RefCell
fn main()
{
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![])
    });

    println!("Leaf parent: {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)])
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("Leaf parent: {:?}", leaf.parent.borrow().upgrade());
}
