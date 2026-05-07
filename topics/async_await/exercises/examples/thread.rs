use std::thread;
use std::time::Duration;

fn main() {
    // Spawning too many threads can crash this program (OS thread and memory limits)
    let mut handles = vec![]; // To store thread join handles
    for i in 0..1_000_000_000 { // Loop to spawn 1 million threads
        handles.push(std::thread::spawn(move || { // Spawn a new OS thread
            std::thread::sleep(Duration::from_millis(100)); // Simulate work (I/O wait)
            println!("Thread: {} 🍔 is ready", i); // Print when done
        }));
    }

    // Wait for all spawned threads to complete
    for h in handles {
        h.join().unwrap(); // Main thread waits for each spawned thread
    }
}