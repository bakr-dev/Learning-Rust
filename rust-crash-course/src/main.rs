// #![deny(clippy::all)]
// #[derive(PartialEq)]
// #[derive(Debug)]

fn main() {
    println!("\n--- Unwrap with Functions (`unwrap_or_else`) ---");
    // this is closure function
    let expensive_default = || {
        println!("Computing expensive default...");
        // Simulate expensive computation
        std::thread::sleep(std::time::Duration::from_millis(100));
        99
    };

    let val2: Option<i32> = None;
    let val2_result = val2.unwrap_or_else(expensive_default); // Closure IS executed
    println!("Value 2: {}", val2_result);
}
