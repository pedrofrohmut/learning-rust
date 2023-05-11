use std::{sync::{Mutex, Arc}, thread};

fn main()
{
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);

    // Here we want to share the same Mutex to different threads
    // To share a reference we usually use the Rc Smart Pointer but we cant
    // The Arc SmartPointer have the same function as the Rc but is thread safe
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        // shadow the mutex with an Arc clone
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
