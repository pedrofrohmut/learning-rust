use std::{thread, sync::{mpsc, Arc, Mutex}};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
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
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        // 1. Arc is the share smart pointer that's thread safe to share
        // the same receiver between all the workers
        // 2. Mutex is to garantee the Job is accessed by only one worker
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender }
    }

    // FnOnce: The closure passed is expected to be runned once
    // Send: So the clojure can be passed from one thread to another
    // 'static: We dont know how long a thread will take to execute
    pub fn execute<F>(&self, job_clojure: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(job_clojure);
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,

}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker
    {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {id} got a job; executing.");
            job();
        });
        Worker { id, thread }
    }
}
