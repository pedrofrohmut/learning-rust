#![allow(dead_code)]
#![allow(unused_variables)]

use foo::{Draw, Screen, Button};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>
}

impl Draw for SelectBox {
    fn draw(&self) {}
}

fn main()
{
    let screen = Screen {
        components: vec![
            // You cannot use here because it does not impl Draw
            Box::new(String::from("Test")),

            Box::new(SelectBox {
                width: 100,
                height: 100,
                options: vec![
                    String::from("yes"),
                    String::from("no"),
                    String::from("maybe")
                ]
            }),

            Box::new(Button {
                width: 100,
                height: 100,
                label: String::from("ok")
            })
        ]
    };
}
