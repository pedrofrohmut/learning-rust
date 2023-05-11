#![allow(dead_code)]
#![allow(unused_variables)]

struct CustomSmartPointer {
    data: String
}

// You implement the Drop trait when you want a custom behavior to when drop is called
impl Drop for CustomSmartPointer {
    fn drop(&mut self)
    {
        println!("Dropping CustomSmartPointer with data: {}", self.data);
    }
}

fn main()
{
    let c = CustomSmartPointer {
        data: String::from("my stuff")
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff")
    };

    println!("CustomSmartPointers created.");

    // Rust does not allow the destructor to be called directly
    // to avoid doble frees 'c.drop();'

    // But you can Still drop it with Rust std lib drop
    drop(c);

    println!("End of main here.");
}
