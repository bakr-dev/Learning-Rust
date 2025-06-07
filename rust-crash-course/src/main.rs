// #![deny(clippy::all)]
// #[derive(PartialEq)]
// #[derive(Debug)]

fn main() {
    println!("\n--- Mapping an Option (`map`, `and_then`) ---");
    let initial_value = Some(10);
    let mapped_value = initial_value.map(|x| x * 2); // Some(20)
    println!("Mapped value: {:?}", mapped_value);

    let none_value: Option<i32> = None;
    let mapped_none = none_value.map(|x| x * 2); // None
    println!("Mapped none: {:?}", mapped_none);
}
