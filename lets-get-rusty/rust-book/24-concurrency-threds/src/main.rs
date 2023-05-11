use std::{thread, time::Duration};

fn main()
{
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {} from the Spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..=5 {
        println!("Hi number {} from the Main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    let arr = vec![1, 2, 3];

    // To use the vector arr inside the thread you have to move it
    let handle2 = thread::spawn(move || {
        println!("Here's a vector: {:?}", arr);
    });

    handle2.join().unwrap();
}
