#![allow(dead_code)]
#![allow(unused_variables)]

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

pub fn eat_at_restaurant()
{
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

fn serve_order() {}

mod back_of_house {
    fn cook_order() {}

    fn fix_incorrect_order()
    {
        // Relative path: in the same module, no need for prefix
        cook_order();
        // Relative path: with super you can reference the parent module
        super::serve_order();
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast
        {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // When you mark a Enum a public all of its variants are public as well
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant2()
{
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
}

pub fn eat_at_restaurant3()
{
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
