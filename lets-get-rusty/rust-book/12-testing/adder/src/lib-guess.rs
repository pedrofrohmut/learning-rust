#![allow(dead_code)]
#![allow(unused_variables)]

pub struct Guess {
    value: i32
}

impl Guess {
    pub fn old_new(value: i32) -> Guess
    {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100");
        }
        Guess { value }
    }

    pub fn new(value: i32) -> Guess
    {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1");
        } else if value > 100 {
            panic!("Guess value must be smaller than or equal to 100");
        }
        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Guess value must be smaller than or equal to 100")]
    fn greater_than_100()
    {
        let my_guess = Guess::new(101);
    }

    #[test]
    #[should_panic(expected = "Guess value must be greater than or equal to 1")]
    fn smaller_than_1()
    {
        let my_guess = Guess::new(0);
    }
}
