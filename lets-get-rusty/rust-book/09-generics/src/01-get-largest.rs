fn main()
{
    // Without generics
    let numbers = vec![34, 50, 25, 100, 65];
    let largest = get_largest_number(numbers);
    println!("Largest: {}", largest);

    let chars = vec!['y', 'm', 'a', 'q'];
    let largest = get_largest_char(chars);
    println!("Largest: {}", largest);

    // With generics
    let numbers = vec![34, 50, 25, 100, 65];
    let largest = get_largest(numbers);
    println!("Largest: {}", largest);

    let chars = vec!['y', 'm', 'a', 'q'];
    let largest = get_largest(chars);
    println!("Largest: {}", largest);
}

fn get_largest_number(numbers: Vec<i32>) -> i32
{
    let mut largest = numbers[0];
    for number in numbers {
        if number > largest {
            largest = number;
        }
    }
    return largest;
}

fn get_largest_char(chars: Vec<char>) -> char
{
    let mut largest = chars[0];
    for x in chars {
        if x > largest {
            largest = x
        }
    }
    return largest;
}

// T has to be a type that can be ordered and can be copied
fn get_largest<T: PartialOrd + Copy>(values: Vec<T>) -> T
{
    let mut largest = values[0];
    for x in values {
        if x > largest { largest = x }
    }
    return largest;
}
