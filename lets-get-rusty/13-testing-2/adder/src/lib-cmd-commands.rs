pub fn prints_and_returns_10(a: i32) -> i32
{
    println!("I got the value: {}", a);
    10
}

pub fn add_two(a: i32) -> i32
{
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), String>
    {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    // Dont run my test in parallel: $ cargo test -- --test-threads=1
    #[test]
    fn it_works2()
    {
        assert_eq!(2 + 2, 4);
    }

    // Show output: $ cargo test -- --show-output
    #[test]
    fn this_will_pass()
    {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_shall_not_pass()
    {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }

    #[test]
    fn add_two_and_two()
    {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_two_and_three()
    {
        assert_eq!(5, add_two(3));
    }

    // Runs only one by name: $ cargo test one_hundred
    // Runs the whole module: $ cargo test tests::
    #[test]
    fn one_hundred()
    {
        assert_eq!(102, add_two(100));
    }

    // $ cargo test -- --ignored
    #[test]
    #[ignore]
    fn expensive_test()
    {
        // code here ....
    }
}
