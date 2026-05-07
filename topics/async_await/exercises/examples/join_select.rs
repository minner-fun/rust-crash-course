use std::time::Duration; // Required import for Duration
use tokio::{join, select}; // Ensure macros are imported


// Assume other necessary tokio imports like tokio::time::sleep are present

// Simulates an async task that completes after `dt` milliseconds
async fn make(val: &'static str, dt: u64) -> &'static str {
    tokio::time::sleep(Duration::from_millis(dt)).await;
    val
}



#[tokio::main]
async fn main() {
    println!("Starting join! example...");
    let start_time = std::time::Instant::now();

    let (res1, res2, res3) = join!(
        make("coffee", 100),    // Simulates a task taking 100ms
        make("green tea", 50), // Simulates a task taking 50ms
        make("lemonade", 20)   // Simulates a task taking 20ms
    );

    println!("join! completed in: {:?}", start_time.elapsed());
    println!("join: res1 = {:?}", res1);
    println!("join: res2 = {:?}", res2);
    println!("join: res3 = {:?}", res3);

    // ... (select! example will follow here)

    println!("\nStarting select! example...");
    let start_time_select = std::time::Instant::now();

    let res = select! {
        val = make("coffee", 20) => {
            println!("select!: 'coffee' (100ms) future finished first");
            val // This `val` is "coffee"
        },
        val = make("green tea", 20) => {
            println!("select!: 'green tea' (50ms) future finished first");
            val // This `val` is "green tea"
        },
        val = make("lemonade", 20) => {
            println!("select!: 'lemonade' (20ms) future finished first");
            val // This `val` is "lemonade"
        },
    };

    println!("select! completed in: {:?}", start_time_select.elapsed());
    println!("select: res = {:?}", res);
}