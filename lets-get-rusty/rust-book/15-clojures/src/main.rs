#![allow(dead_code)]
#![allow(unused_variables)]

use std::{thread, time::Duration};

fn simulated_expensive_calculation(intensity: u32) -> u32
{
    print!("Calculating stuff...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

struct Cacher<T> where T: Fn(u32) -> u32 {
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T>
    {
        Cacher { calculation, value: None }
    }

    fn value(&mut self, arg: u32) -> u32
    {
        match self.value {
            Some(value) => value,
            None => {
                let value = (self.calculation)(arg);
                self.value = Some(value);
                value
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32)
{
    let mut cached_result = Cacher::new(|num: u32| -> u32 {
        println!("Calculating stuff...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups", cached_result.value(intensity));
        println!("Next, do {} situps", cached_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today. Remember to stay hydrated");
        } else {
            println!("Today, run for {} minutes", cached_result.value(intensity));
        }
    }
}

fn main()
{
    let simulated_intensity = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_intensity, simulated_random_number);
}
