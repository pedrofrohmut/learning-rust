use std::{thread, sync::{mpsc, Arc, Mutex}};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// Create a new ThreadPool
    ///
    /// The size is the number of threads in the pool
    ///
    /// # Panics
    ///
    /// The 'new' function will panic if the size is zero
    pub fn new(size: usize) -> ThreadPool
    {
        // 0 is valid usize but is does not make sense
        assert!(size > 0);

        // A channel to communicate with the thread inside the worker and assign jobs
        let (sender, receiver) = mpsc::channel();

        // 1. Arc is the share smart pointer that's thread safe to share
        // the same receiver between all the workers
        // 2. Mutex is to garantee the Job is accessed by only one worker
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender: Some(sender) }
    }

    // FnOnce: The closure passed is expected to be runned once
    // Send: So the clojure can be passed from one thread to another
    // 'static: We dont know how long a thread will take to execute
    pub fn execute<F>(&self, job_clojure: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(job_clojure);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self)
    {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker: {}", worker.id);
            // 1. Using take to claim ownership of the thread and leaving a None
            // in its former place
            // 2. If let to destruct the Some(val) => val returned by the take function
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    // Option here so the thread ownership can be passed for clean up
    thread: Option<thread::JoinHandle<()>>,

}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker
    {
        let thread = thread::spawn(move || loop {
            // 1. Lock so no other worker will be assing with the same thread
            // 2. Recv so it will wait until the receiver to return a value
            let message = receiver.lock().unwrap().recv();

            // When the sender is dropped recv will return an error that will be
            // used to break the infinite loop inside the thread
            match message {
                Ok(job) => {
                    println!("Worker {id} got a job. Executing...");
                    job();
                },
                Err(_) => {
                    println!("Worker {id} disconnected. Shutting Down...");
                    break;
                }
            }
        });

        Worker { id, thread: Some(thread) }
    }
}
