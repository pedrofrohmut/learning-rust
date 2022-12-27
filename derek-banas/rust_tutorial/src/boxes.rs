#![allow(unused)]

use std::io;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use rand::Rng;

fn main()
{
    let my_box = Box::new(10);
    println!("my_box = {}", my_box);

    struct TreeNode<T> {
        pub left:  Option<Box<TreeNode<T>>>,
        pub right: Option<Box<TreeNode<T>>>,
        pub key:   T
    }

    impl<T> TreeNode<T> {
        pub fn new(key: T) -> Self {
            return TreeNode { left: None, right: None, key };
        }

        pub fn add_left(mut self, node: TreeNode<T>) -> Self {
            self.left = Some(Box::new(node));
            return self;
        }

        pub fn add_right(mut self, node: TreeNode<T>) -> Self {
            self.right = Some(Box::new(node));
            return self;
        }
    }

    let left_node = TreeNode::new(2);
    let right_node = TreeNode::new(3);
    let node1 = TreeNode::new(1).add_left(left_node).add_right(right_node);
}
