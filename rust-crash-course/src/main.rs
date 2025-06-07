// #![deny(clippy::all)]
// #[derive(PartialEq)]
// #[derive(Debug)]

fn main() {
    println!("\n--- Option Method: and() ---");
    let user_id = Some(123);
    let auth_token = Some("abcxyz");
    let result_and = user_id.and(auth_token); // Some("abcxyz")
    println!("Result of and: {:?}", result_and);
}
