pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<T>;
}

struct Counter {}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item>
    {
        Some(0)
    }
}

pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}

struct Counter {}

impl Iterator<u32> for Counter {
    fn next(&mut self) -> Option<u32>
    {
        Some(0)
    }
}

impl Iterator<u16> for Counter {
    fn next(&mut self) -> Option<u16>
    {
        Some(0)
    }
}

fn main()
{
}
