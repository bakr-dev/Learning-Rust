// #![deny(clippy::all)]
// #[derive(PartialEq)]
// #[derive(Debug)]

fn main() {
    println!("Using .into_iter() (owned values):");
    let arr_owned = [100, 200, 300];
    for val in arr_owned.into_iter() {
        // `val` is the owned value
        print!("{} ", val);
    }
    println!();
    println!("Original array after .into_iter(): {:?}", arr_owned); // Error if non-Copy type
}
