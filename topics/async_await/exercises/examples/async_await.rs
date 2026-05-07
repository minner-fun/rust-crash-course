use tokio::time::{sleep, Duration}; // Use tokio's sleep

// Add Tokio as a dependency in Cargo.toml:
// tokio = { version = "1", features = ["full"] }
// And use the tokio::main macro for your main function.

#[tokio::main]
async fn main() {
    let mut handles = vec![]; // To store Tokio task JoinHandles

    for i in 0..3_000_000_00 { // Loop to spawn 1 million async tasks
        // Create an async block (a future)
        let fut = async move {  // move 是把所有变量都移动到fut内
            sleep(Duration::from_millis(100)).await; // Asynchronous sleep
            println!("Async: {} 🍔 is ready", i);
        };
        // Spawn the future as a Tokio task on the runtime
        let handler = tokio::task::spawn(fut);
        handles.push(handler);
    }

    // Wait for all spawned Tokio tasks to complete
    for h in handles {
        h.await.unwrap(); // Await the JoinHandle (which is also a future)
    }
}