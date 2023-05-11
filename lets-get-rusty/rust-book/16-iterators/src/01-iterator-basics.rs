pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // Methods with default implementation elided
}

#[test]
pub fn iterator_next_method()
{
    let v2 = vec![33, 42, 66];

    // iter returns immutable references
    // iter_mut for mutable reference
    // into_iter for own types
    let mut v2_iter = v2.iter();

    assert_eq!(v2_iter.next(), Some(&33));
    assert_eq!(v2_iter.next(), Some(&42));
    assert_eq!(v2_iter.next(), Some(&66));
    assert_eq!(v2_iter.next(), None);
}

#[test]
fn iterator_sum()
{
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total = v1_iter.sum::<i32>();
    assert_eq!(total, 6);
}

#[test]
fn produces_another_iterator()
{
    let v2 = vec![1, 2, 3];
    let v3: Vec<_> = v2.iter().map(|x|  x + 1 ).collect();

    assert_eq!(v3, vec![2, 3, 4]);
}

fn main()
{
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for value in v1_iter { println!("Got: {}", value); }
}
