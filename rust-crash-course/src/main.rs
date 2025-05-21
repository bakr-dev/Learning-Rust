#![deny(clippy::all)]

fn say_hello(name: &String) {
    print!("Hello {}", name)
}
fn main() {
    let name = &String::from("John");
    say_hello(name);
    // Example showing the error case
    // {
    //     let immutable_ref1 = &data2[0];
    //     let mutable_ref2 = &mut data2; // Error: cannot borrow `data2` as mutable because it is also borrowed as immutable
    //     println!("Immutable ref: {}", immutable_ref1);
    //     mutable_ref2.push(40);
    // }
}
