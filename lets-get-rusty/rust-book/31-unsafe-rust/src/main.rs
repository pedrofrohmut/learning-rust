use core::slice;

// The body of unsafe functions are unsafe blocks
unsafe fn dangerous() {}

extern "C" {
    fn abs(input: i32) -> i32;
}

// Global variables: in Rust they are called static variables, have static
// lifetime and the type must be declared
static HELLO_WORLD: &str = "Hello, World!";

// Static variables can be mutable
static mut COUNTER: u32 = 0;

fn add_to_counter(inc: u32)
{
    unsafe {
        COUNTER += inc;
    }
}

// A trait is unsafe when at least one of its methods is unsafe
unsafe trait Foo {
    // ...
}

unsafe impl Foo for i32 {
    // ...
}

fn main()
{
    let mut num = 5;

    // immutable raw pointer
    let r1 = &num as *const i32;

    // mutable raw pointer
    let r2 = &mut num as *mut i32;

    // to derefenrence raw pointers you need an unsafe block
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // Needed when calling unsafe functions
    unsafe {
        dangerous();
    }

    let mut vec = vec![1, 2, 3, 4, 5, 6, 7, 8];

    let r = &mut vec[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6, 7, 8]);

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    add_to_counter(3);
    unsafe {
        println!("Global Counter: {}", COUNTER);
    }
}

fn split_at_mut2(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32])
{
    let len = slice.len();
    assert!(mid <= len);
    (&mut slice[..mid], &mut slice[mid..])
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32])
{
    let len = slice.len();
    // gets a raw mutable pointer to the slice
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    // Instead of making the whole function unsafe make only this part
    unsafe {
        let left = slice::from_raw_parts_mut(ptr, mid);
        let right = slice::from_raw_parts_mut(ptr.add(mid), len - mid);
        (left, right)
    }
}
