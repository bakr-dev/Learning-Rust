// This file provides a comprehensive summary of Rust's `Option<T>` enum,
// a fundamental type for handling the possible absence of a value.
// It covers creation, safe and unsafe unwrapping, manipulation, and common use cases.

fn main() {
    // -------------------------------------------------------------------------
    // Introduction to `Option<T>`
    // -------------------------------------------------------------------------
    // In Rust, the `Option<T>` enum is used to express that a value could
    // either be present (`Some(T)`) or absent (`None`). This forces you, the
    // programmer, to explicitly handle both possibilities, preventing common
    // null pointer errors found in other languages.
    //
    // `Option<T>` is defined in the standard library as:
    // enum Option<T> {
    //     None, // Represents no value
    //     Some(T), // Represents a value of type T
    // }
    //
    // Note: "Optionals" is a general programming concept. In Rust, the specific
    // type used for this concept is `Option<T>`. The terms are often used
    // interchangeably in conversation, but `Option<T>` is the Rust type name.

    // -------------------------------------------------------------------------
    // 1. Creating Options
    // -------------------------------------------------------------------------

    // a. `Some(value)`: When a value is present.
    let some_number = Some(5); // Option<i32>
    let some_string = Some(String::from("hello")); // Option<String>
    println!("Some number: {:?}", some_number);
    println!("Some string: {:?}", some_string);

    // b. `None`: When a value is absent.
    let no_number: Option<i32> = None; // Type annotation is often needed for `None`
    let no_string: Option<String> = None;
    println!("No number: {:?}", no_number);
    println!("No string: {:?}", no_string);

    // -------------------------------------------------------------------------
    // 2. Unwrapping Options Safely
    // -------------------------------------------------------------------------
    // Safe unwrapping methods force you to handle the `None` case, preventing panics.

    // a. Using `match` expression: The most exhaustive and common way.
    println!("\n--- Unwrapping Safely with `match` ---");
    let config_value = Some("debug");
    match config_value {
        Some(value) => println!("Configuration value: {}", value),
        None => println!("Configuration value is missing."),
    }

    let user_input: Option<i32> = None;
    match user_input {
        Some(num) => println!("User entered: {}", num),
        None => println!("No input provided by user."),
    }

    // b. Using `if let`: Concise way to handle only the `Some` case.
    println!("\n--- Unwrapping Safely with `if let` ---");
    let favorite_color = Some(String::from("blue"));
    if let Some(color) = favorite_color {
        println!("My favorite color is {}", color);
    } else {
        println!("I don't have a favorite color.");
    }
    // `favorite_color` is moved into `color` if `Some`.
    // println!("{:?}", favorite_color); // Error: value moved

    let mut optional_score = Some(95);
    if let Some(score) = &mut optional_score {
        // Borrow mutably to modify
        *score += 5;
        println!("Updated score: {}", score);
    }
    println!("Optional score after modification: {:?}", optional_score); // Some(100)

    // -------------------------------------------------------------------------
    // 3. Unwrapping Options Unsafely (and Force Unwrapping)
    // -------------------------------------------------------------------------
    // These methods will panic if the `Option` is `None`. Use them only when
    // you are absolutely certain the `Option` will be `Some`.

    // a. `unwrap()`: Panics with a default message if `None`.
    println!("\n--- Unwrapping Unsafely with `unwrap()` ---");
    let safe_value = Some(42);
    let value = safe_value.unwrap();
    println!("Unwrapped value: {}", value);

    // let dangerous_value: Option<i32> = None;
    // let _ = dangerous_value.unwrap(); // This line would panic at runtime!
    // println!("This line will not be reached if unwrap() panics.");

    // b. `expect(message)`: Panics with a custom message if `None`.
    // Useful for providing more context about why a panic occurred.
    println!("\n--- Unwrapping Unsafely with `expect()` ---");
    let file_content = Some(String::from("File data."));
    let content = file_content.expect("Expected file content, but it was missing!");
    println!("File content: {}", content);

    // let missing_file: Option<String> = None;
    // let _ = missing_file.expect("Failed to read configuration file."); // This line would panic!

    // -------------------------------------------------------------------------
    // 4. Mutating Option Values
    // -------------------------------------------------------------------------
    // To modify the value inside a `Some` variant, you need a mutable `Option`
    // and often use `if let` with a mutable reference.

    println!("\n--- Mutating Option Values ---");
    let mut maybe_count = Some(10);
    if let Some(count) = &mut maybe_count {
        *count += 1; // Dereference the mutable reference to change the value
        println!("Count incremented to: {}", count);
    }
    println!("Final maybe_count: {:?}", maybe_count); // Some(11)

    let mut maybe_name: Option<String> = Some(String::from("Rust"));
    if let Some(name) = &mut maybe_name {
        name.push_str("ace");
        println!("Modified name: {}", name);
    }
    println!("Final maybe_name: {:?}", maybe_name); // Some("Rustace")

    // -------------------------------------------------------------------------
    // 5. Unwrapping Multiple Options (with Tuples or Chaining)
    // -------------------------------------------------------------------------
    // When you have multiple `Option`s that all need to be `Some` for an
    // operation to proceed, you can use nested `match` or `if let`.

    println!("\n--- Unwrapping Multiple Options ---");
    let x_coord = Some(10);
    let y_coord = Some(20);
    let z_coord: Option<i32> = None;

    // a. Nested `match`
    match (x_coord, y_coord, z_coord) {
        // Match on a tuple of Options
        (Some(x), Some(y), Some(z)) => println!("All coordinates present: ({}, {}, {})", x, y, z),
        (Some(x), Some(y), None) => println!("2D coordinates present: ({}, {})", x, y),
        _ => println!("Some coordinates are missing."),
    }

    // b. Chaining with `and_then` (more functional style)
    // `and_then` allows sequencing operations that return `Option`.
    // It only proceeds if the current `Option` is `Some`.
    let result_coords = x_coord.and_then(|x| {
        y_coord.and_then(|y| {
            // If both x and y are Some, then return a new Some with their sum
            Some(x + y)
        })
    });
    println!("Sum of x and y (if both Some): {:?}", result_coords); // Some(30)

    let result_coords_with_none = x_coord.and_then(|x| {
        z_coord.and_then(|z| {
            // This inner `z_coord` is None, so the whole chain becomes None
            Some(x + z)
        })
    });
    println!(
        "Sum of x and z (if both Some): {:?}",
        result_coords_with_none
    ); // None

    // -------------------------------------------------------------------------
    // 6. Unwrap with Default Value (`unwrap_or`, `unwrap_or_default`)
    // -------------------------------------------------------------------------
    // Provides a fallback value if the `Option` is `None`.

    println!("\n--- Unwrap with Default Value ---");
    let user_setting = Some(100);
    let default_setting = user_setting.unwrap_or(50); // If `user_setting` is None, use 50
    println!("User setting: {}", default_setting); // 100

    let admin_setting: Option<u32> = None;
    let fallback_setting = admin_setting.unwrap_or(10);
    println!("Admin setting (with fallback): {}", fallback_setting); // 10

    // `unwrap_or_default()`: Uses the `Default` trait implementation for `T`.
    // Requires `T` to implement `Default`.
    let empty_string_option: Option<String> = None;
    let actual_string = empty_string_option.unwrap_or_default(); // Defaults to `String::new()`
    println!("Actual string (default): '{}'", actual_string);

    let some_vec_option = Some(vec![1, 2]);
    let actual_vec = some_vec_option.unwrap_or_default();
    println!("Actual vec (from Some): {:?}", actual_vec);

    // -------------------------------------------------------------------------
    // 7. Unwrap with Functions (`unwrap_or_else`)
    // -------------------------------------------------------------------------
    // Similar to `unwrap_or`, but the default value is computed by a closure,
    // which is only executed if the `Option` is `None`. This is more efficient
    // if computing the default value is expensive.

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

    // -------------------------------------------------------------------------
    // 8. Checking if Option is Some or None (`is_some`, `is_none`)
    // -------------------------------------------------------------------------
    // Simple boolean checks for the variant.

    println!("\n--- Checking `is_some()` / `is_none()` ---");
    let data_status = Some("Data loaded");
    println!("Is data_status Some? {}", data_status.is_some()); // true
    println!("Is data_status None? {}", data_status.is_none()); // false

    let error_status: Option<&str> = None;
    println!("Is error_status Some? {}", error_status.is_some()); // false
    println!("Is error_status None? {}", error_status.is_none()); // true

    // -------------------------------------------------------------------------
    // 9. Mapping an Option (`map`, `and_then`)
    // -------------------------------------------------------------------------
    // `map()` transforms the value inside `Some` without unwrapping.
    // If the `Option` is `None`, `map` returns `None` without executing the closure.
    // `and_then()` (also known as flat_map) is similar but the closure must return an `Option`.
    // It's used for chaining operations that might also fail (return `None`).

    println!("\n--- Mapping an Option (`map`, `and_then`) ---");
    let initial_value = Some(10);
    let mapped_value = initial_value.map(|x| x * 2); // Some(20)
    println!("Mapped value: {:?}", mapped_value);

    let none_value: Option<i32> = None;
    let mapped_none = none_value.map(|x| x * 2); // None
    println!("Mapped none: {:?}", mapped_none);

    // Example with `and_then`: Simulating a fallible division
    fn safe_divide(numerator: f64, denominator: f64) -> Option<f64> {
        if denominator == 0.0 {
            None
        } else {
            Some(numerator / denominator)
        }
    }

    let initial_num = Some(100.0);
    let divisor = Some(5.0);
    let zero_divisor = Some(0.0);

    let result_chain1 = initial_num.and_then(|n| divisor.and_then(|d| safe_divide(n, d)));
    println!("Result of chained division (success): {:?}", result_chain1); // Some(20.0)

    let result_chain2 = initial_num.and_then(|n| zero_divisor.and_then(|d| safe_divide(n, d)));
    println!("Result of chained division (failure): {:?}", result_chain2); // None

    // -------------------------------------------------------------------------
    // 10. Functions Returning Options
    // -------------------------------------------------------------------------
    // A common and idiomatic use of `Option` is as a return type for functions
    // that might not always produce a result.

    println!("\n--- Functions Returning Options ---");
    fn find_first_vowel(s: &str) -> Option<char> {
        for c in s.chars() {
            if "aeiouAEIOU".contains(c) {
                return Some(c); // Found a vowel, return it
            }
        }
        None // No vowel found
    }

    let word1 = "hello";
    let word2 = "rhythm";
    println!("First vowel in '{}': {:?}", word1, find_first_vowel(word1)); // Some('e')
    println!("First vowel in '{}': {:?}", word2, find_first_vowel(word2)); // None

    // -------------------------------------------------------------------------
    // 11. Additional `Option` Methods and Patterns
    // -------------------------------------------------------------------------

    // a. `filter()`: Transforms `Some(T)` to `None` if the predicate is false.
    println!("\n--- Option Method: filter() ---");
    let age_option = Some(25);
    let adult_age = age_option.filter(|&age| age >= 18);
    println!("Adult age (25): {:?}", adult_age); // Some(25)

    let child_age_option = Some(15);
    let adult_child_age = child_age_option.filter(|&age| age >= 18);
    println!("Adult age (15): {:?}", adult_child_age); // None

    // b. `take()`: Takes the value out of the `Option`, leaving `None` behind.
    // Useful when you want to consume the value and then replace the `Option` with `None`.
    println!("\n--- Option Method: take() ---");
    let mut data_to_process = Some(vec![1, 2, 3]);
    let processed_data = data_to_process.take(); // `data_to_process` becomes `None`
    println!("Processed data: {:?}", processed_data); // Some([1, 2, 3])
    println!("Original option after take: {:?}", data_to_process); // None

    // c. `or()`: Returns the `Option` if `Some`, otherwise returns the other `Option`.
    println!("\n--- Option Method: or() ---");
    let primary_source = Some("data from cache");
    let secondary_source: Option<&str> = None;
    let fallback_source = secondary_source.or(Some("data from database"));
    println!("Fallback source: {:?}", fallback_source); // Some("data from database")

    let preferred_source = primary_source.or(Some("data from network"));
    println!("Preferred source: {:?}", preferred_source); // Some("data from cache")

    // d. `and()`: Returns `None` if either is `None`, otherwise returns the second `Option`.
    println!("\n--- Option Method: and() ---");
    let user_id = Some(123);
    let auth_token = Some("abcxyz");
    let result_and = user_id.and(auth_token); // Some("abcxyz")
    println!("Result of and: {:?}", result_and);

    let no_auth_token: Option<&str> = None;
    let result_and_none = user_id.and(no_auth_token); // None
    println!("Result of and with None: {:?}", result_and_none);

    // -------------------------------------------------------------------------
    // Conclusion: The Power of `Option<T>`
    // -------------------------------------------------------------------------
    // `Option<T>` is a cornerstone of Rust's type system, promoting robust
    // error handling and preventing an entire class of bugs related to null
    // or missing values. By forcing explicit handling of `Some` and `None`
    // variants, Rust helps you write safer and more reliable code.
}
