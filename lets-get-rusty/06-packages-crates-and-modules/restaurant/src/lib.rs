#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

// Define front_of_house here but take the content from a different file
// with the same name as the module
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant()
{
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
