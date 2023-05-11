#![allow(unused_variables)]

fn main()
{
    let sum = my_sum(19, 23);
    println!("The sum is {}", sum)
}

fn my_sum(x: i32, y: i32) -> i32
{
    println!("My function: result is {}", x + y);
    x + y
}
