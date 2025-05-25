#![deny(clippy::all)]
// #[derive(PartialEq)]

fn main() {
    println!("\n--- `map` for Ok Values ---");
    let num_str = "123";
    let parsed_and_doubled = num_str.parse::<i32>().map(|num| num * 2); // Doubles the number if parsing succeeds

    match parsed_and_doubled {
        Ok(val) => println!("Parsed and doubled: {}", val),
        Err(e) => eprintln!("Error parsing: {}", e),
    }

    let bad_num_str = "abc";
    let parsed_and_doubled_err = bad_num_str.parse::<i32>().map(|num| num * 2); // Error passes through
    match parsed_and_doubled_err {
        Ok(val) => println!("Parsed and doubled: {}", val),
        Err(e) => eprintln!("Error parsing 'abc': {}", e),
    }
}
