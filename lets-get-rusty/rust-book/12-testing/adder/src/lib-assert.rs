#![allow(dead_code)]
#![allow(unused_variables)]

pub fn add_two(a: i32) -> i32
{
    a + 2
}

pub fn greeting(name: &str) -> String
{
    format!("Hello, {}", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn  it_adds_two()
    {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name()
    {
        let result = greeting("John");
        assert!(
            result.contains("John"),
            "Greeting did not contains the name {}",
            "John");
    }
}
