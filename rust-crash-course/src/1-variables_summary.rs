// 1- Name Convention in Rust
// Rust favors snake_case for variable names. This means all lowercase letters
// with underscores separating words. This improves readability.
fn main() {
    let number_of_items = 10; // Good: snake_case
    let userName = "Alice";     // Not idiomatic: camelCase
    println!("Number of items: {}", number_of_items);
    println!("User name: {}", userName);
}

// 2- Line of Code End with Semicolon
// In Rust, most statements end with a semicolon (;). This signals the end
// of an expression. Expressions that return a value do not typically end
// with a semicolon (this is common in function return values).
fn add(x: i32, y: i32) -> i32 {
    x + y // No semicolon here, as this is the return expression
}

fn greet(name: &str) {
    println!("Hello, {}!", name); // Semicolon indicates the end of the statement
}

// 3- Rust Detects Data Types Automatically (Type Inference)
// Rust can often infer the data type of a variable based on the value
// assigned to it. However, you can also explicitly specify the type.
let inferred_number = 42;           // Rust infers i32
let inferred_float = 3.14;          // Rust infers f64
let explicit_string: String = String::from("Hello"); // Explicit type annotation

println!("Inferred number: {}", inferred_number);
println!("Inferred float: {}", inferred_float);
println!("Explicit string: {}", explicit_string);

// 4- How to Change Value of Variable (Immutable vs. Mutability)
// By default, variables in Rust are immutable, meaning their value cannot
// be changed after they are bound. To make a variable mutable, you need
// to use the `mut` keyword.
let immutable_value = 5;
// immutable_value = 10; // This would cause a compile-time error

let mut mutable_value = 5;
println!("Initial mutable value: {}", mutable_value);
mutable_value = 10; // This is allowed because `mutable_value` is declared with `mut`
println!("Updated mutable value: {}", mutable_value);

// 5- Type Changing (Not Directly Possible)
// Rust is a statically-typed language, meaning that once a variable has a
// certain type, you cannot directly change its type. You would typically
// create a new variable or use a conversion function (e.g., `as`).
let integer_val = 10;
// let string_val = integer_val; // This would be a type mismatch error

let string_from_int = integer_val.to_string();
println!("Integer as string: {}", string_from_int);

let float_from_int = integer_val as f64; // Explicit type casting
println!("Integer as float: {}", float_from_int);

// 6- Data Type Inference and Explicit Annotation
// As seen in point 3, Rust can often infer types automatically. You can also
// explicitly annotate types before the variable name. Specifying types at
// the end of the variable name (e.g., `let x = 5 i32;`) is not valid for
// variable declarations. However, you can specify the type of a literal
// directly using a suffix (e.g., `20u8`).
let explicit_number: i32 = 100; // Explicit type annotation before the variable name
println!("Explicit number: {}", explicit_number);

// Invalid syntax (type annotation at the end of the variable name):
// let strange_declaration = 200 i64; // This is not valid Rust syntax
// let another_strange_declaration = 200i64; // Also not valid for variable declaration

// Valid syntax for specifying the type of a literal:
let literal_with_type = 20u8;
println!("Literal with explicit type: {}", literal_with_type);

// 7- Integer Types (Which Auto Detected and Which Should Specify)
// Rust's default integer type is `i32`. If you don't specify a type for
// an integer literal, Rust will generally infer `i32`. For other integer
// sizes (like `i8`, `i16`, `i64`, `i128`, `u8`, `u16`, `u32`, `u64`, `u128`)
// or architecture-dependent sizes (`isize`, `usize`), you often need to
// specify the type explicitly, especially when there's a specific reason
// for using a particular size or signedness.
let default_int = 50;           // Inferred as i32
let small_unsigned: u8 = 255;   // Explicitly u8
let large_signed: i64 = -10000000000; // Explicitly i64

println!("Default integer: {}", default_int);
println!("Small unsigned integer: {}", small_unsigned);
println!("Large signed integer: {}", large_signed);

// You can also specify the type directly on the literal:
let literal_u8 = 10_u8;
let literal_i64 = -50_i64;
println!("Literal u8: {}", literal_u8);
println!("Literal i64: {}", literal_i64);

// 8- Operators
// Rust supports standard arithmetic operators (+, -, *, /, %), comparison
// operators (==, !=, >, <, >=, <=), logical operators (&&, ||, !), and
// bitwise operators (&, |, ^, <<, >>).
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

// 9- Variable Shadowing
// Rust allows you to declare a new variable with the same name as a previous
// variable. The new variable *shadows* the previous one. The shadowed
// variable is no longer directly accessible in the current scope.
let shadowed_variable = "initial value";
println!("First shadowed_variable: {}", shadowed_variable);

let shadowed_variable = 5; // This shadows the previous `shadowed_variable`
println!("Second shadowed_variable: {}", shadowed_variable);

let shadowed_variable = shadowed_variable + 10;
println!("Third shadowed_variable: {}", shadowed_variable);

// 10- Shadow with Blocks
// Shadowing is particularly useful within different scopes, like blocks
// (defined by curly braces). When the block ends, the shadowed variable
// within that block goes out of scope, and the outer variable (if any)
// becomes accessible again.
let outer_variable = "outer";
println!("Outer variable before block: {}", outer_variable);

{
    let outer_variable = "inner"; // Shadows the outer variable within this block
    println!("Outer variable inside block: {}", outer_variable);
}

println!("Outer variable after block: {}", outer_variable);

// 11- Constants
// Constants are values that are bound to a name and are not allowed to
// change. They are declared using the `const` keyword. You must annotate
// the type of a constant. Constants can be declared in any scope, including
// global scope.
const MAX_POINTS: u32 = 100_000; // Type annotation is mandatory
println!("Maximum points: {}", MAX_POINTS);

// Note: `let` with an uppercase name is a convention for variables that
// are intended to be immutable but are not compile-time constants.

// 12- Tuples
// A tuple is a fixed-size collection of values of potentially different types.
// Tuples are created by writing a comma-separated list of values inside
// parentheses.
let my_tuple = (500, "hello", 3.14, true);
println!("My tuple: {:?}", my_tuple); // Use {:?} for debug printing tuples

// 13- Access or Read Tuples Data
// You can access elements of a tuple using destructuring or by using the
// dot notation followed by the index (starting from 0).
let data = (1, 2.5, "world");

// Destructuring:
let (first, second, third) = data;
println!("First: {}, Second: {}, Third: {}", first, second, third);

// Dot notation:
println!("First element: {}", data.0);
println!("Second element: {}", data.1);
println!("Third element: {}", data.2);