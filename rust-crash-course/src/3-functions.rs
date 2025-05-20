// This file covers the fundamental concepts of functions in Rust, building on ownership, borrowing, and related concepts.

fn main() {
    // -------------------------------------------------------------------------
    // 1. Function Declaration, Basic Syntax, Parameters, and Arguments
    // -------------------------------------------------------------------------
    // Functions are declared using the `fn` keyword.
    // Parameters are type-annotated, and the return type is specified with `->`.
    // If a function doesn't return a value, it implicitly returns the unit type `()` (an empty tuple).
    // Functions can take parameters, which are special variables that are part of a function’s signature.
    // When a function is called, actual values (arguments) are passed to it.

    fn greet() {
        // or  fn greet() -> () {
        // This function takes no arguments and returns nothing (implicitly `()`). meaning returns unit type.
        println!("Hello from the greet function!");
    }

    greet(); // Calling the function

    fn print_number(x: i32) {
        // `x` is a parameter of type i32
        println!("The number is: {}", x);
    }

    print_number(42); // 42 is the argument passed to `x`

    fn add_numbers(a: i32, b: i32) -> i32 {
        // This function takes two i32 parameters and returns an i32
        a + b // Rust is expression-based; the last expression is implicitly returned.
        // No semicolon means it's an expression, not a statement.
    }

    let sum = add_numbers(10, 20);
    println!("Sum: {}", sum);

    // -------------------------------------------------------------------------
    // 2. Return Values (Expressions vs. Statements)
    // -------------------------------------------------------------------------
    // Functions can return values. In Rust, functions are expression-oriented.
    // The value of the last expression in the function body is returned implicitly.
    // You can also use the `return` keyword for early returns.

    fn five() -> i32 {
        5 // This is an expression; its value (5) is returned.
    }

    let x = five();
    println!("Value from five(): {}", x);

    fn plus_one(x: i32) -> i32 {
        x + 1 // Expression, implicitly returned
    }

    let result = plus_one(5);
    println!("Result of plus_one: {}", result);

    fn divide_or_fail(numerator: f64, denominator: f64) -> Result<f64, String> {
        // Using `Result` for explicit error handling
        if denominator == 0.0 {
            Err(String::from("Cannot divide by zero!"))
        } else {
            Ok(numerator / denominator)
        }
    }

    match divide_or_fail(10.0, 2.0) {
        Ok(val) => println!("Division result: {}", val),
        Err(e) => println!("Error: {}", e),
    }

    match divide_or_fail(10.0, 0.0) {
        Ok(val) => println!("Division result: {}", val),
        Err(e) => println!("Error: {}", e),
    }

    // -------------------------------------------------------------------------
    // 3. Expression-Oriented Nature and Implicit Returns
    // -------------------------------------------------------------------------
    // A key characteristic of Rust functions is their expression-oriented nature.
    // Any expression that is the last line of a function body, and does not have
    // a semicolon (`;`) after it, will be the return value of that function.

    fn calculate_area(width: u32, height: u32) -> u32 {
        width * height // This is an expression. No semicolon. Its value is the return value.
    }

    let area = calculate_area(5, 8);
    println!("Calculated area: {}", area);

    fn check_positive(num: i32) -> bool {
        if num > 0 {
            true // Expression: returns `true`
        } else {
            false // Expression: returns `false`
        }
        // No semicolon after the `if/else` block, so its result is implicitly returned.
    }

    println!("Is 10 positive? {}", check_positive(10));
    println!("Is -5 positive? {}", check_positive(-5));

    // If you add a semicolon to the last expression, it becomes a statement,
    // and the function will implicitly return the unit type `()`.
    // This will lead to a compile-time error if the function signature
    // expects a different return type.

    /*
    fn example_error_function(x: i32) -> i32 {
        x + 1; // Adding a semicolon turns this into a statement.
               // The function implicitly returns `()`, but its signature
               // expects `i32`, causing a "mismatched types" error.
    }
    // Uncommenting the above function and trying to compile would result in:
    // error[E0308]: mismatched types
    //   expected `i32`, found `()`
    */

    // -------------------------------------------------------------------------
    // 4. Ownership and Functions
    // -------------------------------------------------------------------------
    // Understanding how ownership interacts with functions is crucial in Rust.
    // When a non-`Copy` type (like `String`) is passed as an argument, its ownership moves to the function.
    // The original variable becomes invalid after the call.
    // For types that implement the `Copy` trait (e.g., integers, booleans, fixed-size arrays),
    // the value is copied, and the original variable remains valid.

    // A. Passing Ownership (Move)
    fn takes_ownership(some_string: String) {
        // `some_string` now owns the data
        println!("`takes_ownership` received: {}", some_string);
    } // `some_string` goes out of scope and `drop` is called.

    let my_string = String::from("hello from main");
    takes_ownership(my_string); // `my_string`'s ownership moves into `takes_ownership`
    // println!("After takes_ownership: {}", my_string); // Compile-time error: value borrowed here after move

    // B. Copying (for `Copy` types)
    fn makes_copy(some_integer: i32) {
        // `some_integer` is a copy of the original value
        println!("`makes_copy` received: {}", some_integer);
    } // `some_integer` goes out of scope, nothing is dropped

    let my_integer = 100;
    makes_copy(my_integer); // `my_integer`'s value is copied, `my_integer` is still valid
    println!("After makes_copy: {}", my_integer);

    // -------------------------------------------------------------------------
    // 5. Borrowing (References)
    // -------------------------------------------------------------------------
    // To allow a function to use a value without taking ownership, you pass a reference (`&`).
    // This is known as "borrowing." The owner remains valid.
    // To allow a function to modify a value without taking ownership, you pass a mutable reference (`&mut`).
    // The original variable must be mutable. Only one mutable reference can exist at a time.

    // C. Borrowing (References)
    fn calculate_length(s: &String) -> usize {
        // `s` is an immutable reference to a String
        s.len()
    } // `s` goes out of scope, but it doesn't own the data, so no drop occurs

    let s1 = String::from("functional programming");
    let len = calculate_length(&s1); // Pass a reference to `s1`
    println!("The length of '{}' is {}", s1, len); // `s1` is still valid here

    // D. Mutable Borrowing
    fn append_text(s: &mut String) {
        // `s` is a mutable reference, allowing modification
        s.push_str(" and more!");
    } // `s` goes out of scope, but it doesn't own the data

    let mut changeable_string = String::from("initial text");
    append_text(&mut changeable_string); // Pass a mutable reference to `changeable_string`
    println!("Modified string: {}", changeable_string); // `changeable_string` is now modified

    // -------------------------------------------------------------------------
    // 6. Functions Returning Ownership
    // -------------------------------------------------------------------------
    // Functions can create a value and return its ownership, or return ownership of a value they received.

    fn give_ownership() -> String {
        // This function will move its created String to the caller
        let some_string = String::from("owned by function, now by caller");
        some_string // Ownership of `some_string` is moved out
    }

    let gained_ownership = give_ownership();
    println!("Gained ownership: {}", gained_ownership);

    fn take_and_give_back_ownership(a_string: String) -> String {
        // This function takes ownership and then returns it
        println!("`take_and_give_back_ownership` received: {}", a_string);
        a_string // Ownership of `a_string` is moved out
    }

    let original_string = String::from("round trip");
    let returned_string = take_and_give_back_ownership(original_string); // Ownership moves in, then out
    // println!("Original string after round trip: {}", original_string); // Error: original_string moved
    println!("Returned string: {}", returned_string);

    // -------------------------------------------------------------------------
    // 7. Closures (Anonymous Functions)
    // -------------------------------------------------------------------------
    // Rust has closures, which are anonymous functions you can save in a variable or pass to other functions.
    // They can capture values from the scope in which they're defined.
    // Closures can take multiple arguments, just like regular functions.

    let num = 5;
    let add_five = |x: i32| x + num; // Closure capturing `num` from its environment

    let sum_closure = add_five(10);
    println!("Sum with closure: {}", sum_closure);

    // Closures can also take ownership of captured variables (`move` keyword).
    let mut greeting = String::from("hello");
    let print_greeting = move || {
        // `move` forces the closure to take ownership of `greeting`
        println!("{}", greeting);
    };

    print_greeting();
    // println!("{}", greeting); // Error: value borrowed here after move (because print_greeting took ownership)

    // Example of a closure with multiple arguments:
    let multiply = |x: i32, y: i32| -> i32 { x * y };

    let product = multiply(4, 6);
    println!("Product with closure: {}", product);

    // -------------------------------------------------------------------------
    // 8. Higher-Order Functions (Functions as Arguments)
    // -------------------------------------------------------------------------
    // Functions can be passed as arguments to other functions.

    fn apply_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(f(arg))
    }

    fn double(x: i32) -> i32 {
        x * 2
    }

    let doubled_twice = apply_twice(double, 3);
    println!("Doubled twice: {}", doubled_twice);

    // You can also pass closures to functions that expect functions or specific traits.
    let square_closure = |x: i32| x * x;
    let squared_twice = apply_twice(square_closure, 3); // Closures often implement `Fn`, `FnMut`, or `FnOnce` traits.
    println!("Squared twice: {}", squared_twice);
}
