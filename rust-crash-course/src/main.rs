#![deny(clippy::all)]
fn main() {
    let mut value = 10;
    let ref1 = &mut value;
    // let ref2 = &mut value; // Compile-time error: cannot borrow `value` as mutable more than once at a time

    *ref1 += 5;
    println!("Value: {}", value);
}
