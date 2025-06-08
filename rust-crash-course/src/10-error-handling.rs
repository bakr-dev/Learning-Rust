// This file covers fundamental error handling concepts in Rust, focusing on
// `panic!` for unrecoverable errors and `Result` for recoverable errors.

// Import necessary modules for I/O operations
use std::fs; // Provides file system operations like reading and writing files.
use std::io::{self, Read, Write}; // Import io::Error for I/O-related errors, and Read/Write traits for file operations.

// The main function can now return a Result, allowing for error propagation
// from main itself, especially when using the `?` operator.
// `Box<dyn std::error::Error>` is a common way to return any kind of error that
// implements the `Error` trait, without needing to know its exact type.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Print a starting message for the error handling examples.
    println!("--- Starting Rust Error Handling Examples ---");

    // -------------------------------------------------------------------------
    // 1. `panic!` for Unrecoverable Errors
    // -------------------------------------------------------------------------
    // `panic!` is used when a program encounters a serious, unrecoverable
    // error. It immediately stops execution and unwinds the stack, cleaning
    // up data. This is typically used for bugs, unexpected states, or situations
    // where there's no reasonable way to proceed.

    // Uncomment the following lines to see `panic!` in action.
    // let x = 10; // Declare an integer variable x.
    // let y = 0; // Declare an integer variable y, initialized to 0.
    // if y == 0 { // Check if y is zero, which would cause a division by zero error.
    //     panic!("Cannot divide by zero! This is an unrecoverable logic error."); // If y is zero, trigger a panic with a descriptive message.
    // }
    // let result = x / y; // This line would attempt division, leading to a runtime error if y is 0.
    // println!("Result: {}", result); // Print the result (this line would not be reached if a panic occurs).

    // Inform the user that the panic examples are commented out.
    println!("\nProgram continues after potential panic comment.");

    // `panic!` can also be caused by out-of-bounds array access.

    // -------------------------------------------------------------------------
    // 2. `Result` for Recoverable Errors: The `enum` for Success or Failure
    // -------------------------------------------------------------------------
    // `Result<T, E>` is an enum that represents either success (`Ok(T)`) or
    // failure (`Err(E)`). `T` is the type of the value returned on success,
    // and `E` is the type of the error returned on failure.
    // This is Rust's primary mechanism for handling recoverable errors.

    // A simple function that might fail
    fn safe_divide(numerator: f64, denominator: f64) -> Result<f64, String> {
        // Define a function `safe_divide` that takes two f64 numbers and returns a Result.
        // On success, it returns an `f64`; on failure, it returns a `String` containing the error message.
        if denominator == 0.0 {
            // Check if the denominator is zero.
            Err(String::from("Division by zero is not allowed.")) // If it is, return an `Err` variant with an error message.
        } else {
            // If the denominator is not zero.
            Ok(numerator / denominator) // Return an `Ok` variant with the result of the division.
        }
    }

    // -------------------------------------------------------------------------
    // 3. Handling `Result` with `match`
    // -------------------------------------------------------------------------
    // The `match` expression is the most common way to handle `Result` types.
    // It allows you to explicitly handle both the `Ok` and `Err` variants.

    // Print a header for the `match` example.
    println!("\n--- Handling Result with `match` ---");

    let division_result_ok = safe_divide(10.0, 2.0); // Call `safe_divide` with valid inputs.
    match division_result_ok {
        // Use a `match` expression to handle the `Result`.
        Ok(value) => println!("Successful division: {}", value), // If the result is `Ok`, print the successful value.
        Err(error) => println!("Error during division: {}", error), // If the result is `Err`, print the error message.
    }

    let division_result_err = safe_divide(10.0, 0.0); // Call `safe_divide` with inputs that will cause an error.
    match division_result_err {
        // Use a `match` expression to handle the `Result`.
        Ok(value) => println!("Successful division: {}", value), // If the result is `Ok` (unlikely here), print the value.
        Err(error) => println!("Error during division: {}", error), // If the result is `Err`, print the error message.
    }

    // -------------------------------------------------------------------------
    // 4. `unwrap()` and `expect()`: Panicking on Error (Use with Caution!)
    // -------------------------------------------------------------------------
    // `unwrap()` and `expect()` are convenience methods on `Result` that
    // extract the `Ok` value or `panic!` if the `Result` is an `Err`.
    // `expect()` allows you to provide a custom panic message.
    // These should generally be avoided in production code unless you are
    // absolutely certain the operation will succeed, or if a failure
    // genuinely indicates an unrecoverable bug that should crash the program.

    // Print a header for `unwrap()` and `expect()` examples.
    println!("\n--- `unwrap()` and `expect()` (Use with Caution!) ---");

    // This would panic if the file doesn't exist:
    // let file_content = fs::read_to_string("this_file_does_not_exist.txt"); // Attempt to read a non-existent file.
    // let content = file_content.unwrap(); // `unwrap()` will panic if `file_content` is an `Err`.
    // println!("Content (unwrap): {}", content); // This line would not be reached if a panic occurs.

    // This would panic with a custom message if the file doesn't exist:
    // let content = fs::read_to_string("non_existent_file.txt").expect("Failed to read the file! I expected this file to exist."); // Attempt to read a non-existent file and use `expect()` for a custom panic message.
    // println!("Content (expect): {}", content); // This line would not be reached if a panic occurs.

    // Inform the user that the `unwrap()` and `expect()` examples are commented out.
    println!("`unwrap()` and `expect()` examples commented out to prevent panics.");

    // A relatively safe use of `unwrap()` or `expect()`: when parsing a known valid number.
    let parsed_number = "42".parse::<i32>().unwrap(); // Parse a string "42" into an `i32`. `unwrap()` is safe here because "42" is a valid integer.
    println!("Parsed number (safe unwrap): {}", parsed_number); // Print the parsed number.

    // -------------------------------------------------------------------------
    // 5. Void Result Values or Errors (`Result<(), E>` or `Result<T, ()>`)
    // -------------------------------------------------------------------------
    // Sometimes, a function might not return any meaningful value on success,
    // only indicating that an operation completed successfully or failed.
    // In such cases, `()` (the unit type) is used as `T` in `Result<(), E>`.
    // Similarly, if the error itself carries no specific information, `()` can
    // be used as `E` in `Result<T, ()>`.

    // Define a function `create_empty_file` that creates an empty file.
    // It returns `Result<(), io::Error>`: `Ok(())` on success (no specific value), or an `io::Error` on failure.
    fn create_empty_file(path: &str) -> Result<(), io::Error> {
        fs::File::create(path)?; // Attempt to create a file at the given path. The `?` operator propagates any `io::Error`.
        println!("Successfully created empty file: {}", path); // Print a success message if the file is created.
        Ok(()) // Return `Ok(())` to indicate successful completion without a specific value.
    }

    // Print a header for void Result examples.
    println!("\n--- Void Result Values ---");
    let file_to_create = "my_empty_file.txt"; // Define the name of the file to create.
    match create_empty_file(file_to_create) {
        // Call `create_empty_file` and handle its `Result`.
        Ok(_) => println!("File creation operation reported success."), // If `Ok(())` is returned, print a success message.
        Err(e) => eprintln!("File creation failed: {}", e), // If an `Err` is returned, print the error to standard error.
    }

    // Clean up the dummy file
    let _ = fs::remove_file(file_to_create); // Attempt to remove the created file. `_` is used to ignore the `Result` of `remove_file`.

    // -------------------------------------------------------------------------
    // 6. The `?` Operator for Error Propagation (Early Exit)
    // -------------------------------------------------------------------------
    // The `?` operator is a concise way to propagate errors, enabling "early exit".
    // When placed after an expression that returns a `Result`:
    // 1. If the `Result` is `Ok`, its inner value is unwrapped, and execution continues.
    // 2. If the `Result` is `Err`, the error value is immediately returned from the
    //    *current function*. This makes it behave like an early `return Err(error)`.
    // The `?` operator requires the current function's return type to be compatible
    // with the error type being propagated.

    // Define a function to read a username from a file.
    // It returns `Result<String, io::Error>`: `Ok(String)` on success, or an `io::Error` on failure.
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut f = fs::File::open("username.txt")?; // Attempt to open "username.txt". If it fails, the error is immediately returned from this function. If successful, `f` holds the file handle.
        let mut contents = String::new(); // Create an empty `String` to store file contents.
        f.read_to_string(&mut contents)?; // Attempt to read the file's contents into `contents`. If it fails, the error is immediately returned.
        Ok(contents) // If both operations succeed, return the `contents` wrapped in `Ok`.
    }

    // Let's create a dummy file for the `read_username_from_file` example
    let _ = fs::write("username.txt", "Rusty_Dev"); // Create a file named "username.txt" with content "Rusty_Dev". `_` ignores the `Result`.

    // Print a header for the `?` operator example.
    println!("\n--- `?` Operator for Error Propagation ---");

    match read_username_from_file() {
        // Call `read_username_from_file` and handle its `Result`.
        Ok(username) => println!("Username from file: {}", username), // If successful, print the username.
        Err(e) => eprintln!("Error reading username: {}", e), // If an error occurs, print it to standard error.
    }

    let _ = fs::remove_file("username.txt"); // Clean up the created "username.txt" file.

    // Example of `?` causing an early exit in `main` (requires `main` to return `Result`)
    println!("\n--- `?` in main (Early Exit) ---");
    let content_from_non_existent = fs::read_to_string("another_non_existent.txt"); // Attempt to read a non-existent file. This returns a `Result`.
    if content_from_non_existent.is_err() {
        // Check if the `Result` is an `Err`.
        eprintln!(
            "Attempted to read non-existent file in main. The `?` operator would have returned here if not handled by if/else."
        ); // Print a message indicating what would happen with `?`.
        // If we uncommented the line below, main would return Err directly
        // let _content = fs::read_to_string("another_non_existent.txt")?; // This `?` would immediately return the `io::Error` from `main`.
    }

    // -------------------------------------------------------------------------
    // 7. `map` and `map_err` for Transforming `Result` Values
    // -------------------------------------------------------------------------
    // These methods allow you to transform the value inside an `Ok` or `Err`
    // variant without unwrapping the `Result` itself.

    // `map`: Transforms the `Ok` value. If the Result is `Err`, it's passed through unchanged.
    // Print a header for `map` example.
    println!("\n--- `map` for Ok Values ---");
    let num_str = "123"; // A string representing a number.
    let parsed_and_doubled = num_str.parse::<i32>().map(|num| num * 2); // Attempt to parse the string to an `i32`. If successful (`Ok`), apply the lambda `|num| num * 2` to double the number. If `Err`, the error is passed through.

    match parsed_and_doubled {
        // Handle the transformed `Result`.
        Ok(val) => println!("Parsed and doubled: {}", val), // If `Ok`, print the doubled value.
        Err(e) => eprintln!("Error parsing: {}", e),        // If `Err`, print the parsing error.
    }

    let bad_num_str = "abc"; // A string that cannot be parsed as a number.
    let parsed_and_doubled_err = bad_num_str.parse::<i32>().map(|num| num * 2); // Attempt to parse "abc". This will result in an `Err`. The `map` operation will be skipped.
    match parsed_and_doubled_err {
        // Handle the transformed `Result`.
        Ok(val) => println!("Parsed and doubled: {}", val), // This branch will not be taken.
        Err(e) => eprintln!("Error parsing 'abc': {}", e), // This branch will be taken, printing the parsing error for "abc".
    }

    // `map_err`: Transforms the `Err` value. If the Result is `Ok`, it's passed through unchanged.
    // Print a header for `map_err` example.
    println!("\n--- `map_err` for Err Values ---");
    let potentially_failing_op: Result<i32, u32> = Err(404); // Create a `Result` that is an `Err` with a `u32` error code.
    let transformed_error = potentially_failing_op.map_err(|err_code| {
        // Use `map_err` to transform the `u32` error into a `String` error.
        format!("Failed with error code: {}", err_code) // Transforms u32 error into a String error.
    });

    match transformed_error {
        // Handle the transformed `Result`.
        Ok(val) => println!("Operation successful: {}", val), // This branch will not be taken.
        Err(e) => eprintln!("Operation failed with custom error message: {}", e), // This branch will be taken, printing the custom error message.
    }

    let successful_op: Result<i32, u32> = Ok(10); // Create a `Result` that is `Ok` with an `i32` value.
    let transformed_ok =
        successful_op.map_err(|err_code| format!("Failed with error code: {}", err_code)); // Use `map_err`. Since the original `Result` is `Ok`, `map_err` does nothing, and the `Ok` value passes through.
    match transformed_ok {
        // Handle the transformed `Result`.
        Ok(val) => println!("Operation successful (error not transformed): {}", val), // This branch will be taken.
        Err(e) => eprintln!("Operation failed: {}", e), // This branch will not be taken.
    }

    // -------------------------------------------------------------------------
    // 8. Custom Error Types
    // -------------------------------------------------------------------------
    // For more complex applications, you'll often define your own custom
    // error types using enums to represent different kinds of errors your
    // functions might encounter. This provides more specific and meaningful
    // error information.

    #[derive(Debug)] // Derive the `Debug` trait so we can print instances of `MyError` using `{:?}`.
    enum MyError {
        // Define a custom error enum `MyError`.
        NotFound,             // Variant for "not found" errors.
        PermissionDenied,     // Variant for "permission denied" errors.
        InvalidInput(String), // Variant for invalid input, holding a `String` with more details.
        Io(io::Error), // Add a variant to wrap `io::Error`, allowing it to be part of our custom error type.
    }

    // Implement `From<io::Error>` for `MyError` to allow `?` operator to work
    // when `io::Error` needs to be converted into `MyError`.
    impl From<io::Error> for MyError {
        // Implement the `From` trait, which allows automatic conversion from `io::Error` to `MyError`.
        fn from(error: io::Error) -> Self {
            // The `from` function takes an `io::Error`.
            MyError::Io(error) // It constructs a `MyError::Io` variant, wrapping the `io::Error`.
        }
    }

    // Define a function that performs some risky operation and returns a `Result` with our custom error type.
    fn do_something_risky(value: i32) -> Result<String, MyError> {
        if value == 0 {
            // If the value is 0, return a `NotFound` error.
            Err(MyError::NotFound)
        } else if value < 0 {
            // If the value is negative, return a `PermissionDenied` error.
            Err(MyError::PermissionDenied)
        } else if value > 100 {
            // If the value is greater than 100, return an `InvalidInput` error with a custom message.
            Err(MyError::InvalidInput(format!(
                "Value {} is too large.",
                value
            )))
        } else {
            // If the value is between 1 and 100 (inclusive).
            // Example of using a `?` operator that converts `io::Error` to `MyError`
            let file_name = format!("data_{}.txt", value); // Create a filename based on the value.
            fs::write(&file_name, format!("Some data for {}", value))?; // Attempt to write to the file. If `fs::write` returns an `io::Error`, the `?` operator converts it to `MyError::Io` and returns it from `do_something_risky`.
            Ok(format!("Operation successful with value: {}", value)) // If successful, return a success message.
        }
    }

    // Print a header for custom error types.
    println!("\n--- Custom Error Types ---");

    match do_something_risky(0) {
        // Test `do_something_risky` with a value that causes `NotFound`.
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("Risky operation failed: {:?}", e), // Print the error using debug formatting.
    }

    match do_something_risky(-5) {
        // Test `do_something_risky` with a value that causes `PermissionDenied`.
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("Risky operation failed: {:?}", e), // Print the error.
    }

    match do_something_risky(150) {
        // Test `do_something_risky` with a value that causes `InvalidInput`.
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("Risky operation failed: {:?}", e), // Print the error.
    }

    match do_something_risky(50) {
        // Test `do_something_risky` with a valid value.
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("Risky operation failed: {:?}", e), // This branch should not be taken.
    }

    // Clean up files created by do_something_risky
    let _ = fs::remove_file("data_50.txt"); // Remove the file created by the successful call.

    // -------------------------------------------------------------------------
    // 9. Main Function Returning Result (`fn main() -> Result<(), E>`)
    // -------------------------------------------------------------------------
    // As seen at the beginning of this `main` function, Rust allows `main` to
    // return a `Result<(), E>`. This is extremely useful because it allows
    // you to use the `?` operator directly in `main`, propagating any errors
    // from your utility functions up to the program's entry point.
    // If `main` returns `Ok(())`, the program exits successfully.
    // If `main` returns `Err(E)`, the program prints the error and exits with a non-zero status code.
    // `Box<dyn std::error::Error>` is a common choice for the error type `E`
    // because it can represent any type that implements the `Error` trait,
    // making it flexible for various error origins.

    // Print a header for the main function returning `Result` explanation.
    println!("\n--- Main function returning Result ---");
    println!("This entire program is an example of `main` returning `Result`.");

    // Define a helper function to perform an I/O operation.
    fn perform_io_operation() -> Result<(), io::Error> {
        let mut file = fs::File::create("example.txt")?; // Create a file. The `?` operator propagates `io::Error` if creation fails.
        file.write_all(b"Hello, Rust!")?; // Write bytes to the file. The `?` operator propagates `io::Error` if writing fails.
        println!("Successfully wrote to example.txt"); // Print success message.
        Ok(()) // Return `Ok(())` on success.
    }

    // Call the function; if it returns an Err, `main` will automatically
    // return that error because its signature allows it.
    perform_io_operation()?; // Call the I/O operation. If it returns an `Err`, the `?` operator will propagate it out of `main`, causing the program to exit with an error.

    // Clean up the created file
    let _ = fs::remove_file("example.txt"); // Remove the "example.txt" file.

    // Print a completion message.
    println!("\n--- All examples completed successfully. ---");

    // Return Ok(()) to indicate successful execution of the main function.
    Ok(()) // Return `Ok(())` to signify that the `main` function completed without errors.
}
