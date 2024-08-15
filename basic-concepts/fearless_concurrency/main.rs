use std::thread;

fn main() {
    // Create a vector to hold the thread handles.
    let mut handles = vec![];

    // Spawn 5 threads.
    for i in 1..=5 {
        // Each thread will print its index.
        let handle = thread::spawn(move || {
            println!("Hello from thread {}", i);
        });
        handles.push(handle);
    }

    // Join all threads, ensuring they finish before the main thread exits.
    for handle in handles {
        handle.join().unwrap();
    }

    println!("All threads have finished.");
}

