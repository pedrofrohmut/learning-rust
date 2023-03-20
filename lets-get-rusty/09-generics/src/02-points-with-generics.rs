#![allow(dead_code)]
#![allow(unused_variables)]

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}


#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W>
    {
        Point { x: self.x, y: other.y }
    }
}

struct Point2<T> {
    x: T,
    y: T,
}

impl<T> Point2<T> {
    fn get_x(&self) -> &T { &self.x }
}

// Get y is only available for f64 pointers
impl Point2<f64> {
    fn get_y(&self) -> f64 { self.y }
}

fn main()
{
    let point1 = Point { x: 1,   y: 2   };
    let point2 = Point { x: 1.0, y: 2.0 };
    let point3 = Point { x: 1,   y: 3.0 };

    let point4 = Point2 { x: 1,   y: 2   };
    let point5 = Point2 { x: 1.0, y: 2.0 };

    let x4 = point4.get_x();
    // Get y is only available for f64 pointers
    // let y4 = point4.get_y();

    let x5 = point5.get_x();
    let y5 = point5.get_y();

    let point6 = Point { x: 5, y: 10.4 };
    let point7 = Point { x: "hello", y: 'A' };
    let point8 = point6.mixup(point7);
    println!("{:?}", point8);
}
