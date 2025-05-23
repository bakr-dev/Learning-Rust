#![deny(clippy::all)]
// #[derive(PartialEq)]

fn main() {
    println!("\n--- Unwrapping Unsafely with `unwrap()` ---");
    let safe_value = Some(42);
    let value = safe_value.unwrap();
    println!("Unwrapped value: {}", value);
}
