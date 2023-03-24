#![allow(dead_code)]
#![allow(unused_variables)]

fn two_plus_two() -> Result<(), String>
{
    if 2 + 2 == 4 {
        Ok(())
    } else {
        Err(String::from("2 + 2 is not 4"))
    }
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
            Err(String::from("2 + 2 is not 4"))
        }
    }

    #[test]
    fn fn_returns_result_enum() -> Result<(), String>
    {
        two_plus_two()
    }
}
