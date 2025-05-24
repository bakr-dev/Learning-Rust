// This file covers fundamental error handling concepts in Rust, focusing on
// `panic!` for unrecoverable errors and `Result` for recoverable errors.

fn main() {
    // -------------------------------------------------------------------------
    // 1. `panic!` for Unrecoverable Errors
    // -------------------------------------------------------------------------
    // `panic!` is used when a program encounters a serious, unrecoverable
    // error. It immediately stops execution and unwinds the stack, cleaning
    // up data. This is typically used for bugs, unexpected states, or situations
    // where there's no reasonable way to proceed.

    // let x = 10;
    // let y = 0;
    // if y == 0 {
    //     panic!("Cannot divide by zero!"); // This would cause a program crash
    // }
    // let result = x / y;
    // println!("Result: {}", result);

    println!("Program continues after potential panic comment.");

    // `panic!` can also be caused by out-of-bounds array access (demonstrated
    // in the previous variable concepts file).

    // -------------------------------------------------------------------------
    // 2. `Result` for Recoverable Errors: The `enum` for Success or Failure
    // -------------------------------------------------------------------------
    // `Result<T, E>` is an enum that represents either success (`Ok(T)`) or
    // failure (`Err(E)`). `T` is the type of the value returned on success,
    // and `E` is the type of the error returned on failure.
    // This is Rust's primary mechanism for handling recoverable errors.

    // A simple function that might fail
    fn safe_divide(numerator: f64, denominator: f64) -> Result<f64, String> {
        if denominator == 0.0 {
            Err(String::from("Division by zero is not allowed.")) // Return an error
        } else {
            Ok(numerator / denominator) // Return a successful value
        }
    }

    // -------------------------------------------------------------------------
    // 3. Handling `Result` with `match`
    // -------------------------------------------------------------------------
    // The `match` expression is the most common way to handle `Result` types.
    // It allows you to explicitly handle both the `Ok` and `Err` variants.

    println!("\n--- Handling Result with `match` ---");

    let division_result_ok = safe_divide(10.0, 2.0);
    match division_result_ok {
        Ok(value) => println!("Successful division: {}", value),
        Err(error) => println!("Error during division: {}", error),
    }

    let division_result_err = safe_divide(10.0, 0.0);
    match division_result_err {
        Ok(value) => println!("Successful division: {}", value),
        Err(error) => println!("Error during division: {}", error),
    }

    // -------------------------------------------------------------------------
    // 4. `unwrap()` and `expect()`: Panicking on Error (Use with Caution!)
    // -------------------------------------------------------------------------
    // `unwrap()` and `expect()` are convenience methods on `Result` that
    // extract the `Ok` value or `panic!` if the `Result` is an `Err`.
    // `expect()` allows you to provide a custom panic message.
    // These should generally be avoided in production code unless you are
    // absolutely certain the operation will succeed, or if a failure
    // genuinely indicates an unrecoverable bug.

    println!("\n--- `unwrap()` and `expect()` (Use with Caution!) ---");

    let file_content = std::fs::read_to_string("this_file_does_not_exist.txt");

    // This would panic if the file doesn't exist:
    // let content = file_content.unwrap();
    // println!("Content (unwrap): {}", content);

    // This would panic with a custom message if the file doesn't exist:
    // let content = file_content.expect("Failed to read the file!");
    // println!("Content (expect): {}", content);

    println!("`unwrap()` and `expect()` examples commented out to prevent panics.");

    // A safe use of `unwrap()` (e.g., parsing a known valid number)
    let parsed_number = "42".parse::<i32>().unwrap();
    println!("Parsed number (safe unwrap): {}", parsed_number);

    // -------------------------------------------------------------------------
    // 5. The `?` Operator for Error Propagation
    // -------------------------------------------------------------------------
    // The `?` operator is a concise way to propagate errors. When placed
    // after a `Result` value, it will:
    // 1. If the `Result` is `Ok`, unwrap the value and continue.
    // 2. If the `Result` is `Err`, return the error from the *current* function.
    // This is incredibly useful for chaining operations that can fail.
    // Note: The `?` operator can only be used in functions that return a `Result`.

    fn read_username_from_file() -> Result<String, std::io::Error> {
        let mut f = std::fs::File::open("hello.txt")?; // Propagates error if file opening fails
        let mut contents = String::new();
        f.read_to_string(&mut contents)?; // Propagates error if reading fails
        Ok(contents)
    }

    // Let's create a dummy file for the `read_username_from_file` example
    let dummy_file_result = std::fs::write("hello.txt", "Rust User");
    if let Err(e) = dummy_file_result {
        eprintln!("Error creating dummy file: {}", e);
    }

    println!("\n--- `?` Operator for Error Propagation ---");

    match read_username_from_file() {
        Ok(username) => println!("Username from file: {}", username),
        Err(e) => println!("Error reading username: {}", e),
    }

    // Clean up the dummy file
    let _ = std::fs::remove_file("hello.txt");

    // -------------------------------------------------------------------------
    // 6. Custom Error Types (Brief Introduction)
    // -------------------------------------------------------------------------
    // For more complex applications, you'll often define your own custom
    // error types using enums to represent different kinds of errors your
    // functions might encounter. This provides more specific and meaningful
    // error information.

    #[derive(Debug)] // Required for printing with {:?}
    enum MyError {
        NotFound,
        PermissionDenied,
        InvalidInput(String),
    }

    fn do_something_risky(value: i32) -> Result<String, MyError> {
        if value == 0 {
            Err(MyError::NotFound)
        } else if value < 0 {
            Err(MyError::PermissionDenied)
        } else if value > 100 {
            Err(MyError::InvalidInput(format!(
                "Value {} is too large.",
                value
            )))
        } else {
            Ok(format!("Operation successful with value: {}", value))
        }
    }

    println!("\n--- Custom Error Types ---");

    match do_something_risky(0) {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("Risky operation failed: {:?}", e),
    }

    match do_something_risky(-5) {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("Risky operation failed: {:?}", e),
    }

    match do_something_risky(150) {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("Risky operation failed: {:?}", e),
    }

    match do_something_risky(50) {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("Risky operation failed: {:?}", e),
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// This file covers fundamental error handling concepts in Rust, focusing on
// `panic!` for unrecoverable errors and `Result` for recoverable errors.

// Import necessary modules for I/O operations
use std::fs;
use std::io::{self, Read}; // Import io::Error and Read trait

// The main function can now return a Result, allowing for error propagation
// from main itself, especially when using the `?` operator.
// `Box<dyn std::error::Error>` is a common way to return any kind of error that
// implements the `Error` trait, without needing to know its exact type.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Starting Error Handling Examples ---");

    // -------------------------------------------------------------------------
    // 1. `panic!` for Unrecoverable Errors
    // -------------------------------------------------------------------------
    // `panic!` is used when a program encounters a serious, unrecoverable
    // error. It immediately stops execution and unwinds the stack, cleaning
    // up data. This is typically used for bugs, unexpected states, or situations
    // where there's no reasonable way to proceed.

    // let x = 10;
    // let y = 0;
    // if y == 0 {
    //     panic!("Cannot divide by zero! This is an unrecoverable logic error."); // This would cause a program crash
    // }
    // let result = x / y;
    // println!("Result: {}", result);

    println!("\nProgram continues after potential panic comment.");

    // `panic!` can also be caused by out-of-bounds array access (demonstrated
    // in the previous variable concepts file).

    // -------------------------------------------------------------------------
    // 2. `Result` for Recoverable Errors: The `enum` for Success or Failure
    // -------------------------------------------------------------------------
    // `Result<T, E>` is an enum that represents either success (`Ok(T)`) or
    // failure (`Err(E)`). `T` is the type of the value returned on success,
    // and `E` is the type of the error returned on failure.
    // This is Rust's primary mechanism for handling recoverable errors.

    // A simple function that might fail
    fn safe_divide(numerator: f64, denominator: f64) -> Result<f64, String> {
        if denominator == 0.0 {
            Err(String::from("Division by zero is not allowed.")) // Return an error
        } else {
            Ok(numerator / denominator) // Return a successful value
        }
    }

    // -------------------------------------------------------------------------
    // 3. Handling `Result` with `match` and Expecting Error/Value
    // -------------------------------------------------------------------------
    // The `match` expression is the most common way to handle `Result` types.
    // It allows you to explicitly handle both the `Ok` and `Err` variants.

    println!("\n--- Handling Result with `match` ---");

    let division_result_ok = safe_divide(10.0, 2.0);
    match division_result_ok {
        Ok(value) => println!("Successful division: {}", value),
        Err(error) => println!("Error during division: {}", error), // Expecting an error (but got Ok here)
    }

    let division_result_err = safe_divide(10.0, 0.0);
    match division_result_err {
        Ok(value) => println!("Successful division: {}", value),
        Err(error) => println!("Error during division: {}", error), // Expecting an error (and got it here)
    }

    // -------------------------------------------------------------------------
    // 4. `unwrap()` and `expect()`: Panicking on Error (Use with Caution!)
    //    Expecting a value from a Result (and panicking if it's an error)
    // -------------------------------------------------------------------------
    // `unwrap()` and `expect()` are convenience methods on `Result` that
    // extract the `Ok` value or `panic!` if the `Result` is an `Err`.
    // `expect()` allows you to provide a custom panic message.
    // These should generally be avoided in production code unless you are
    // absolutely certain the operation will succeed, or if a failure
    // genuinely indicates an unrecoverable bug that should crash the program.

    println!("\n--- `unwrap()` and `expect()` (Use with Caution!) ---");

    // Example where we "expect a value" (i.e., expect Ok) and panic if it's an error.
    // This part is commented out to allow the program to run without crashing.
    /*
    let content = fs::read_to_string("non_existent_file.txt").expect("Failed to read the file! I expected this file to exist.");
    println!("Content (expect): {}", content);
    */
    println!("`expect()` example commented out to prevent panics for non-existent file.");

    // A relatively safe use of `unwrap()` or `expect()`: when parsing a known valid number.
    let parsed_number = "42".parse::<i32>().unwrap(); // We know "42" is a valid i32, so unwrap is "safe" here.
    println!("Parsed number (safe unwrap): {}", parsed_number);

    // -------------------------------------------------------------------------
    // 5. Void Result Values or Errors (`Result<(), E>` or `Result<T, ()>`)
    // -------------------------------------------------------------------------
    // Sometimes, a function might not return any meaningful value on success,
    // only indicating that an operation completed successfully or failed.
    // In such cases, `()` (the unit type) is used as `T` in `Result<(), E>`.
    // Similarly, if the error itself carries no specific information, `()` can
    // be used as `E` in `Result<T, ()>`.

    fn create_empty_file(path: &str) -> Result<(), io::Error> {
        fs::File::create(path)?; // The `?` operator handles the `io::Error` here.
        println!("Successfully created empty file: {}", path);
        Ok(()) // Return Ok(()) to indicate success without a value
    }

    println!("\n--- Void Result Values ---");
    let file_to_create = "my_empty_file.txt";
    match create_empty_file(file_to_create) {
        Ok(_) => println!("File creation operation reported success."),
        Err(e) => eprintln!("File creation failed: {}", e),
    }

    // Clean up the dummy file
    let _ = fs::remove_file(file_to_create);

    // -------------------------------------------------------------------------
    // 6. Early Exit from Result Errors: The `?` Operator
    // -------------------------------------------------------------------------
    // The `?` operator is a concise way to propagate errors, enabling "early exit".
    // When placed after an expression that returns a `Result`:
    // 1. If the `Result` is `Ok`, its inner value is unwrapped, and execution continues.
    // 2. If the `Result` is `Err`, the error value is immediately returned from the
    //    *current function*. This makes it behave like an early `return Err(error)`.
    // The `?` operator requires the current function's return type to be compatible
    // with the error type being propagated.

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut f = fs::File::open("username.txt")?; // Early exit if file open fails
        let mut contents = String::new();
        f.read_to_string(&mut contents)?; // Early exit if reading fails
        Ok(contents)
    }

    // Let's create a dummy file for the `read_username_from_file` example
    let _ = fs::write("username.txt", "Rusty_Dev");

    println!("\n--- `?` Operator for Early Exit ---");

    match read_username_from_file() {
        Ok(username) => println!("Username from file: {}", username),
        Err(e) => eprintln!("Error reading username: {}", e), // Use eprintln for errors
    }

    let _ = fs::remove_file("username.txt"); // Clean up

    // Example of `?` causing an early exit in `main` (requires `main` to return `Result`)
    println!("\n--- `?` in main (Early Exit) ---");
    let content_from_non_existent = fs::read_to_string("another_non_existent.txt");
    if content_from_non_existent.is_err() {
        eprintln!(
            "Attempted to read non-existent file in main. The `?` operator would have returned here if not handled by if/else."
        );
        // If we uncommented the line below, main would return Err directly
        // let _content = fs::read_to_string("another_non_existent.txt")?; // This would cause main to return an error
    }

    // -------------------------------------------------------------------------
    // 7. `map` and `map_err` for Transforming `Result` Values
    // -------------------------------------------------------------------------
    // These methods allow you to transform the value inside an `Ok` or `Err`
    // variant without unwrapping the `Result` itself.

    // `map`: Transforms the `Ok` value. If the Result is `Err`, it's passed through unchanged.
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

    // `map_err`: Transforms the `Err` value. If the Result is `Ok`, it's passed through unchanged.
    println!("\n--- `map_err` for Err Values ---");
    let potentially_failing_op: Result<i32, u32> = Err(404);
    let transformed_error = potentially_failing_op.map_err(|err_code| {
        format!("Failed with error code: {}", err_code) // Transforms u32 error into a String error
    });

    match transformed_error {
        Ok(val) => println!("Operation successful: {}", val),
        Err(e) => eprintln!("Operation failed with custom error message: {}", e),
    }

    let successful_op: Result<i32, u32> = Ok(10);
    let transformed_ok =
        successful_op.map_err(|err_code| format!("Failed with error code: {}", err_code));
    match transformed_ok {
        Ok(val) => println!("Operation successful (error not transformed): {}", val),
        Err(e) => eprintln!("Operation failed: {}", e),
    }

    // -------------------------------------------------------------------------
    // 8. Main Function Returning Result (`fn main() -> Result<(), E>`)
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

    println!("\n--- Main function returning Result ---");
    println!("This entire program is an example of `main` returning `Result`.");

    // Example of a function called from main that returns a Result
    fn perform_io_operation() -> Result<(), io::Error> {
        let mut file = fs::File::create("example.txt")?;
        file.write_all(b"Hello, Rust!")?;
        println!("Successfully wrote to example.txt");
        Ok(())
    }

    // Call the function; if it returns an Err, `main` will automatically
    // return that error because its signature allows it.
    perform_io_operation()?; // This `?` will propagate the error out of main if it fails.

    // Clean up the created file
    let _ = fs::remove_file("example.txt");

    println!("\n--- All examples completed successfully. ---");

    // Return Ok(()) to indicate successful execution of the main function.
    Ok(())
}
