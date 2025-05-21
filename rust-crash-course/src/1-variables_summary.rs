// This file covers fundamental variable concepts in Rust, including naming conventions,
// mutability, data types, and related topics.

fn main() {
    // -------------------------------------------------------------------------
    // 1. Naming Conventions: snake_case
    // -------------------------------------------------------------------------
    // Rust favors snake_case for variable and function names. This means using
    // lowercase letters with underscores separating words. This convention
    // promotes code readability and consistency.

    let number_of_items = 10; // Good: snake_case
    // let userName = "Alice";    // Not idiomatic: camelCase.  Rust style is all lowercase with underscores.
    let user_name = "Alice"; // Corrected to snake_case
    println!("Number of items: {}", number_of_items);
    println!("User name: {}", user_name);

    // -------------------------------------------------------------------------
    // 2. Statement Endings: Semicolons
    // -------------------------------------------------------------------------
    // In Rust, most statements end with a semicolon (;). This signals the end
    // of an expression. Expressions that return a value, especially in
    // function return values, do not typically end with a semicolon.

    fn add(x: i32, y: i32) -> i32 {
        x + y // No semicolon here, as this is the return expression
    }

    fn greet(name: &str) {
        println!("Hello, {}!", name); // Semicolon indicates the end of the statement
    }

    let sum_result = add(5, 3);
    println!("Sum: {}", sum_result);
    greet("Bob");

    // -------------------------------------------------------------------------
    // 3. Type Inference
    // -------------------------------------------------------------------------
    // Rust can often infer the data type of a variable based on the value
    // assigned to it. This reduces verbosity and improves code readability.
    // However, you can also explicitly specify the type when needed or for clarity.

    let inferred_number = 42; // Rust infers i32
    let inferred_float = 3.14; // Rust infers f64
    let explicit_string: String = String::from("Hello"); // Explicit type annotation

    println!("Inferred number: {}", inferred_number);
    println!("Inferred float: {}", inferred_float);
    println!("Explicit string: {}", explicit_string);

    // -------------------------------------------------------------------------
    // 4. Immutability and Mutability
    // -------------------------------------------------------------------------
    // By default, variables in Rust are immutable, meaning their value cannot
    // be changed after they are bound. This helps prevent accidental data
    // modification and improves code safety. To make a variable mutable,
    // you need to use the `mut` keyword.

    let immutable_value = 5;
    // immutable_value = 10; // This would cause a compile-time error: cannot assign twice to immutable variable

    let mut mutable_value = 5;
    println!("Initial mutable value: {}", mutable_value);
    mutable_value = 10; // This is allowed because `mutable_value` is declared with `mut`
    println!("Updated mutable value: {}", mutable_value);

    // -------------------------------------------------------------------------
    // 5. Preventing Type Changes
    // -------------------------------------------------------------------------
    // Rust is a statically-typed language. Once a variable is assigned a
    // specific type, that type cannot be changed directly.  You can, however,
    // create a new variable with a different type, often using a conversion
    // function or casting.

    let integer_val = 10;
    // let string_val = integer_val; // This would be a type mismatch error: expected `String`, found integer

    let string_from_int = integer_val.to_string(); // Convert integer to String
    println!("Integer as string: {}", string_from_int);

    let float_from_int = integer_val as f64; // Explicit type casting (coercion)
    println!("Integer as float: {}", float_from_int);

    // -------------------------------------------------------------------------
    // 6. Explicit Type Annotation
    // -------------------------------------------------------------------------
    // While Rust often infers types, you can always explicitly annotate them.
    // Type annotations are placed before the variable name, following a colon.
    // You can also specify the type of a literal value using a suffix.

    let explicit_number: i32 = 100; // Explicit type annotation before the variable name
    println!("Explicit number: {}", explicit_number);

    // Invalid syntax (type annotation at the end of the variable name):
    // let strange_declaration = 200 i64; // This is not valid Rust syntax
    // let another_strange_declaration = 200i64; // Also not valid for variable declaration

    // Valid syntax for specifying the type of a literal:
    let literal_u8 = 20u8;
    let literal_i64 = -50i64;
    println!("Literal u8: {}", literal_u8);
    println!("Literal i64: {}", literal_i64);

    // -------------------------------------------------------------------------
    // 7. Integer Type Specification
    // -------------------------------------------------------------------------
    // Rust's default integer type is `i32`. If you don't specify a type for
    // an integer literal, Rust will generally infer `i32`. For other integer
    // sizes (like `i8`, `i16`, `i64`, `i128`, `u8`, `u16`, `u32`, `u64`, `u128`)
    // or architecture-dependent sizes (`isize`, `usize`), you often need to
    // specify the type explicitly, especially when there's a specific reason
    // for using a particular size or signedness, or when the compiler cannot infer the type.

    let default_int = 50; // Inferred as i32
    let small_unsigned: u8 = 255; // Explicitly u8
    let large_signed: i64 = -10000000000; // Explicitly i64

    println!("Default integer: {}", default_int);
    println!("Small unsigned integer: {}", small_unsigned);
    println!("Large signed integer: {}", large_signed);

    // You can also specify the type directly on the literal:
    let literal_u8_suffix = 10_u8;
    let literal_i64_suffix = -50_i64;
    println!("Literal u8: {}", literal_u8_suffix);
    println!("Literal i64: {}", literal_i64_suffix);

    // -------------------------------------------------------------------------
    // 8. Operators
    // -------------------------------------------------------------------------
    // Rust supports standard arithmetic operators (+, -, *, /, %), comparison
    // operators (==, !=, >, <, >=, <=), logical operators (&&, ||, !), and
    // bitwise operators (&, |, ^, <<, >>).  Operator overloading is possible
    // via traits.
    let a = 10;
    let b = 3;

    println!("a + b = {}", a + b);
    println!("a - b = {}", a - b);
    println!("a * b = {}", a * b);
    println!("a / b = {}", a / b);
    println!("a % b = {}", a % b);
    println!("a > b is {}", a > b);
    println!("a == b is {}", a == b);
    println!("!(a > b) is {}", !(a > b));

    // -------------------------------------------------------------------------
    // 9. Variable Shadowing
    // -------------------------------------------------------------------------
    // Rust allows you to declare a new variable with the same name as a
    // previous variable. The new variable *shadows* the previous one.
    // The shadowed variable is no longer directly accessible in the current
    // scope. Shadowing is different from mutability. Shadowing creates a new variable.

    let shadowed_variable = "initial value";
    println!("First shadowed_variable: {}", shadowed_variable);

    let shadowed_variable = 5; // This shadows the previous `shadowed_variable`
    println!("Second shadowed_variable: {}", shadowed_variable);

    let shadowed_variable = shadowed_variable + 10;
    println!("Third shadowed_variable: {}", shadowed_variable);

    // -------------------------------------------------------------------------
    // 10. Shadowing and Scope
    // -------------------------------------------------------------------------
    // Shadowing is particularly useful within different scopes, such as blocks
    // (defined by curly braces). When the block ends, the shadowed variable
    // within that block goes out of scope, and the outer variable (if any)
    // becomes accessible again.

    let outer_variable = "outer";
    println!("Outer variable before block: {}", outer_variable);

    {
        let outer_variable = "inner"; // Shadows the outer variable within this block
        println!("Outer variable inside block: {}", outer_variable);
    } // The inner 'outer_variable' goes out of scope here

    println!("Outer variable after block: {}", outer_variable); // Prints the original "outer"

    // -------------------------------------------------------------------------
    // 11. Constants
    // -------------------------------------------------------------------------
    // Constants are values that are bound to a name and are not allowed to
    // change. They are declared using the `const` keyword. You must annotate
    // the type of a constant. Constants can be declared in any scope,
    // including global scope.  Constants must be initialized with a value
    // that can be determined at compile time.

    const MAX_POINTS: u32 = 100_000; // Type annotation is mandatory
    println!("Maximum points: {}", MAX_POINTS);

    // Note: `let` with an uppercase name is a convention for variables that
    // are intended to be immutable but are not compile-time constants.
    //  Constants are implicitly static.

    // -------------------------------------------------------------------------
    // 12. Tuples
    // -------------------------------------------------------------------------
    // A tuple is a fixed-size, ordered collection of values of potentially
    // different types. Tuples are created by writing a comma-separated list
    // of values inside parentheses.  Tuples can be useful for returning
    // multiple values from a function.
    let my_tuple = (500, "hello", 3.14, true);
    println!("My tuple: {:?}", my_tuple); // Use {:?} for debug printing tuples

    // -------------------------------------------------------------------------
    // 13. Accessing Tuple Data
    // -------------------------------------------------------------------------
    // You can access elements of a tuple using destructuring or by using the
    // dot notation followed by the index (starting from 0).

    let data_tuple = (1, 2.5, "world");

    // Destructuring:
    let (first, second, third) = data_tuple;
    println!("First: {}, Second: {}, Third: {}", first, second, third);

    // Dot notation:
    println!("First element: {}", data_tuple.0);
    println!("Second element: {}", data_tuple.1);
    println!("Third element: {}", data_tuple.2);

    // -------------------------------------------------------------------------
    // 14. Arrays
    // -------------------------------------------------------------------------
    // An array is a collection of values of the *same type*, stored in a
    // fixed-size list. Arrays are useful when you know the exact number of
    // elements you need at compile time. They are allocated on the stack.

    // Declaring an array with inferred type and size
    let numbers = [1, 2, 3, 4, 5];
    println!("Entire array: {:?}", numbers);
    println!("First element: {}", numbers[0]);
    println!("Last element: {}", numbers[4]);

    // Declaring an array with explicit type and size
    let bytes: [u8; 3] = [255, 128, 0];
    println!("Bytes array: {:?}", bytes);

    // Declaring an array with repeated initial value
    let zeroes = [0; 5]; // An array of 5 zeroes
    println!("Zeroes array: {:?}", zeroes);

    // Arrays are immutable by default, like other variables
    let mut mutable_array = [10, 20, 30];
    println!("Mutable array before change: {:?}", mutable_array);
    mutable_array[0] = 5; // Modifying an element (requires `mut`)
    println!("Mutable array after change: {:?}", mutable_array);

    // Attempting to access an out-of-bounds index will cause a runtime panic
    // println!("Out of bounds access: {}", numbers[5]); // This would panic at runtime
}
