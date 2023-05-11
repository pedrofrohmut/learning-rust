use std::{sync::mpsc, thread, time::Duration};

fn main()
{
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        let msg = String::from("Hello, World!");
        sender.send(msg).unwrap();
    });

    // Block current thread execution while wait for receiver to return a value
    let received = receiver.recv().unwrap();
    println!("Got: {}", received);

    let (sender, receiver) = mpsc::channel();

    let sender2 = sender.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("Hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread")
        ];
        for val in vals {
            sender.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("More"),
            String::from("messages"),
            String::from("for"),
            String::from("you")
        ];
        for val in vals {
            sender2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in receiver {
        println!("Got: {}", received);
    }
}
