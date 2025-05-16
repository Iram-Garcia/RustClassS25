// //Assignment 1: Spawning Threads and Joining Them
// use std::thread;
// use std::time::Duration;

// fn main() {
//     println!("Main thread starting");
    
//     // TODO: Create a vector to store thread handles
//     let mut handles = vec![];
    
//     // TODO: Spawn 3 threads
//     for i in 1..=3 {
//         // TODO: Spawn a thread and store its handle
//         let handle = thread::spawn(move || {
//             // Simulate some work
//             println!("Thread {} starting", i);
//             thread::sleep(Duration::from_millis(500));
//             println!("Thread {} finished", i);
//         });
        
//         // TODO: Store the handle
//         handles.push(handle);
//     }
    
//     // TODO: Wait for all threads to complete
    
    
//     println!("All threads completed.");
// }

// //Assignment 2: Sharing Counter Data Between Threads
// use std::sync::{Arc, Mutex};
// use std::thread;

// fn main() {
//     // TODO: Create a shared counter using Arc and Mutex
//     let counter = Arc::new(Mutex::new(0));
//     println!("starting the main thread");
//     // TODO: Create a vector to store thread handles
//     let mut handles = vec![];
    
//     // TODO: Spawn 5 threads
//     for i in 1..=5 {
//         // TODO: Clone the Arc for the thread
//         let thread_counter = Arc::clone(&counter);
        
//         // TODO: Spawn a thread that increments the counter 10 times
//         let handle = thread::spawn(move || {
//             // TODO: Increment counter 10 times
//             for j in 0..10 {
//                 // Lock the mutex to access the counter
//                 let mut num = thread_counter.lock().unwrap();
//                 *num += 1;
//                 println!("Thread {} incremented counter to {}", i, *num);
//             }
//         });
        
//         handles.push(handle);
//     }
    
//     // TODO: Wait for all threads to complete
//     println!("Waiting for all threads to complete...");
//     for handle in handles {
//         handle.join().unwrap();
//     }
    
//     // TODO: Print the final value of the counter
//     let final_count = *counter.lock().unwrap();
//     println!("Final counter value: {}", final_count);
    
// }

//Assignment 3: Thread Pool Implementation
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

// Message to be sent to the workers
enum Message {
    NewJob(Job),
    Terminate,
}

// Job type is a boxed closure that can be sent across threads
type Job = Box<dyn FnOnce() + Send + 'static>;

// ThreadPool struct
struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    // Create a new ThreadPool with the specified size
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        
        // TODO: Create a channel for sending jobs
        let (sender, receiver) = mpsc::channel();
        
        // TODO: Create and store workers
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            // Create a new worker and push it to the vector
            workers.push(Worker::new(id, Arc::new(Mutex::new(receiver.clone()))));
        }
        // TODO: Return the ThreadPool
        Return ThreadPool {
            workers,
            sender,
        }
    }
    
    // Execute a job in the thread pool
    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // TODO: Create a job from the closure and send it to a worker
        
    }
}

// Clean up resources when ThreadPool is dropped
impl Drop for ThreadPool {
    fn drop(&mut self) {
        // TODO: Send terminate message to all workers
        
        
        // TODO: Wait for all workers to finish
        
    }
}

// Worker struct represents a thread that can process jobs
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    // Create a new worker with the specified ID
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        // TODO: Create a thread that loops and receives jobs from the channel
        
        
        // TODO: Return the Worker
        
    }
}

fn main() {
    // Create a new thread pool with 4 workers
    let pool = ThreadPool::new(4);
    
    // Submit 10 tasks to the pool
    for i in 1..=10 {
        pool.execute(move || {
            println!("Processing task {}", i);
            thread::sleep(std::time::Duration::from_millis(500));
            println!("Completed task {}", i);
        });
    }
    
    println!("Main thread waiting for tasks to complete...");
    // ThreadPool will be dropped when it goes out of scope, triggering the cleanup
}