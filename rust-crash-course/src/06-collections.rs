// This file covers the fundamental concepts of common collection types in Rust.
// Collections are data structures that can hold multiple values. Unlike built-in
// array or tuple types, collections store their data on the heap, meaning
// the amount of data they hold doesn't need to be known at compile time and
// can grow or shrink as the program runs.

fn main() {
    // Note: In Rust, arrays and tuples are fundamental, fixed-size compound types
    // that typically reside on the stack. They are NOT considered "collections"
    // in the same way as `Vec`, `String`, or `HashMap`, which are dynamic,
    // heap-allocated data structures.

    // -------------------------------------------------------------------------
    // 1. Vectors (`Vec<T>`)
    // -------------------------------------------------------------------------
    // A `Vec<T>` is a growable list of values of the same type.
    // It stores its data contiguously in memory, making element access efficient.

    // a. Creating Vectors
    // Create an empty vector
    let mut v: Vec<i32> = Vec::new(); // Explicit type annotation needed for empty Vec
    println!("Empty vector: {:?}", v);

    // Create a vector with initial values using the `vec!` macro
    let mut v2 = vec![1, 2, 3]; // Type `Vec<i32>` is inferred
    println!("Initial vector: {:?}", v2);

    // b. Adding Elements
    v.push(5);
    v.push(6);
    v.push(7);
    println!("Vector after pushes: {:?}", v);

    // c. Accessing Elements
    // Access by index (returns a reference, panics if index is out of bounds)
    let third: &i32 = &v[2];
    println!("The third element is: {}", third);

    // Access using `get` method (returns `Option<&T>`, safe for out-of-bounds)
    match v.get(1) {
        Some(second) => println!("The second element is: {}", second),
        None => println!("There is no second element."),
    }

    // Iterating over elements
    println!("Iterating over v:");
    for i in &v {
        println!("{}", i);
    }

    // Iterating and modifying elements (requires mutable reference)
    println!("Iterating and modifying v2:");
    for i in &mut v2 {
        *i += 10; // Dereference `i` to modify the value it points to
    }
    println!("Modified v2: {:?}", v2);

    // d. Dropping a Vector
    // When a vector goes out of scope, it and all its elements are dropped.
    {
        let _v3 = vec![10, 20, 30]; // _v3 is created
    } // _v3 goes out of scope and is dropped here.

    // -------------------------------------------------------------------------
    // 2. Strings (`String`)
    // -------------------------------------------------------------------------
    // In Rust, strings are more complex than in some other languages due to
    // UTF-8 encoding and ownership. There are two main string types:
    // - `String`: A growable, heap-allocated, owned, UTF-8 encoded string.
    // - `&str`: A string slice, which is a reference to a `String` or a string literal.

    // a. Creating Strings
    let mut s1 = String::new(); // Empty mutable String
    println!("Empty string: '{}'", s1);

    let s2 = String::from("initial content"); // From a string literal
    println!("String from literal: '{}'", s2);

    let s3 = "literal".to_string(); // Convert string literal to String
    println!("String from .to_string(): '{}'", s3);

    // b. Appending to a String
    s1.push_str("hello"); // Append a string slice
    println!("After push_str: '{}'", s1);

    s1.push('!'); // Append a single character
    println!("After push char: '{}'", s1);

    let s4 = String::from("Rust");
    let s5 = String::from(" is great!");
    // `+` operator takes ownership of the left-hand side (`s4`)
    // and borrows the right-hand side (`&s5`).
    let s6 = s4 + &s5; // s4 is moved here, can't be used after this line
    println!("Concatenated string: '{}'", s6);
    // println!("s4: {}", s4); // Error: value borrowed here after move

    // Use `format!` macro for complex string concatenation without taking ownership
    let s7 = String::from("Tic");
    let s8 = String::from("Tac");
    let s9 = String::from("Toe");
    let s10 = format!("{}-{}-{}", s7, s8, s9); // s7, s8, s9 are not moved
    println!("Formatted string: '{}'", s10);
    println!("s7: {}, s8: {}, s9: {}", s7, s8, s9); // Still valid

    // c. String Slices (`&str`)
    // String slices are references to a part of a `String` or a string literal.
    let hello = &s6[0..5]; // Slice from index 0 to 5 (exclusive)
    println!("String slice: '{}'", hello);

    // d. Iterating over Strings (Unicode considerations)
    println!("Iterating over characters in '{}':", s10);
    for c in s10.chars() {
        // Iterates over Unicode scalar values
        println!("{}", c);
    }
    // Note: Direct indexing into a String (e.g., `s10[0]`) is not allowed
    // because characters can be multiple bytes in UTF-8, making indexing ambiguous.

    // -------------------------------------------------------------------------
    // 3. Hash Maps (`HashMap<K, V>`)
    // -------------------------------------------------------------------------
    // A `HashMap<K, V>` stores mappings from keys of type `K` to values of type `V`.
    // Keys must be hashable and implement `Eq`.

    use std::collections::HashMap; // Must bring HashMap into scope

    // a. Creating Hash Maps
    let mut scores: HashMap<String, i32> = HashMap::new();
    println!("Empty hash map: {:?}", scores);

    // b. Inserting Key-Value Pairs
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("Hash map after inserts: {:?}", scores);

    // c. Accessing Values
    let team_name = String::from("Blue");
    let score = scores.get(&team_name); // `get` returns `Option<&V>`
    match score {
        Some(s) => println!("Score for {}: {}", team_name, s),
        None => println!("Team not found."),
    }

    // d. Iterating over Hash Maps
    println!("Iterating over scores:");
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // e. Updating Values
    // Overwriting a value
    scores.insert(String::from("Blue"), 25);
    println!("Scores after updating Blue: {:?}", scores);

    // Only insert if key has no value (`entry` method)
    scores.entry(String::from("Red")).or_insert(30); // "Red" not present, inserted
    scores.entry(String::from("Blue")).or_insert(60); // "Blue" present, not updated
    println!("Scores after entry().or_insert(): {:?}", scores);

    // Updating a value based on the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // `or_insert` returns a mutable reference
        *count += 1; // Dereference to increment the value
    }
    println!("Word counts: {:?}", map);

    // f. Ownership with Hash Maps
    // For types that implement `Copy` (like `i32`), values are copied into the hash map.
    // For owned types (like `String`), values are moved into the hash map.
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map2 = HashMap::new();
    map2.insert(field_name, field_value);
    // println!("field_name: {}", field_name); // Error: value moved
    // println!("field_value: {}", field_value); // Error: value moved
    println!("Map2: {:?}", map2);
}
