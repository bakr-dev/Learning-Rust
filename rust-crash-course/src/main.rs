#![deny(clippy::all)]
// #[derive(PartialEq)]

fn main() {
    println!("\n--- Unwrap with Functions (`unwrap_or_else`) ---");
    let expensive_default = || {
        println!("Computing expensive default...");
        // Simulate expensive computation
        std::thread::sleep(std::time::Duration::from_millis(100));
        99
    };

    let val1 = Some(50).unwrap_or_else(expensive_default); // Closure not executed
    println!("Value 1: {}", val1);

    let val2: Option<i32> = None;
    let val2_result = val2.unwrap_or_else(expensive_default); // Closure IS executed
    println!("Value 2: {}", val2_result);
}
