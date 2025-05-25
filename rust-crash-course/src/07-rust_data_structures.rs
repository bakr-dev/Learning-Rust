// This file provides a comprehensive comparison of Rust's fundamental data structures:
// Arrays, Tuples, Vectors, Strings, and Hash Maps, along with an in-depth look at Iterators.
// It covers their creation, common manipulations, typical use cases, and how iterators
// interact with them, highlighting differences in memory allocation, mutability, and ownership.

use std::collections::HashMap; // Required for HashMap

fn main() {
    // -------------------------------------------------------------------------
    // Introduction to Data Structures & Iterators in Rust
    // -------------------------------------------------------------------------
    // Rust offers various ways to organize data. Understanding their core
    // differences, especially regarding memory allocation (stack vs. heap)
    // and mutability, is crucial for writing efficient and safe Rust code.
    // Arrays and Tuples are fixed-size, compile-time known, stack-allocated compound types.
    // Vectors, Strings, and Hash Maps are dynamic, growable, heap-allocated collections.
    // Iterators provide a powerful, lazy, and efficient way to process sequences of data.

    // -------------------------------------------------------------------------
    // 1. Arrays (`[T; N]`) - Fixed-size, Homogeneous, Stack-allocated
    // -------------------------------------------------------------------------
    // Arrays are fixed-size collections of elements of the *same type*.
    // Their length is known at compile time and cannot change. They are
    // typically stack-allocated, making them very efficient for small,
    // known-size data.

    println!("\n--- 1. Arrays (`[T; N]`) ---");

    // a. Creating Arrays
    let fixed_array: [i32; 5] = [1, 2, 3, 4, 5]; // Explicit type and size
    let initialized_array = [0; 3]; // Array of 3 elements, all initialized to 0
    println!("Fixed array: {:?}", fixed_array);
    println!("Initialized array: {:?}", initialized_array);

    // b. Accessing Values (by index)
    let first_val = fixed_array[0];
    println!("First element: {}", first_val);
    // Note: Accessing an index out of bounds (e.g., `fixed_array[10]`)
    // will cause a runtime panic.

    // c. Getting Length
    println!("Length of fixed_array: {}", fixed_array.len());

    // d. Iterating over Arrays
    // Arrays implement `IntoIterator` for `&[T]`, `&mut [T]`, and `[T; N]` (by value).
    // This means you can use `for` loops directly or call `.iter()`, `.iter_mut()`, `.into_iter()`.

    println!("Iterating over fixed_array (immutable references via .iter()):");
    for &val in fixed_array.iter() {
        // `iter()` yields `&T`. `&val` dereferences the reference.
        print!("{} ", val);
    }
    println!("\nOriginal array after .iter(): {:?}", fixed_array); // Array is still usable

    println!("Iterating and modifying mutable_array (mutable references via .iter_mut()):");
    let mut mutable_array = [1, 2, 3];
    for val_ref in mutable_array.iter_mut() {
        // `iter_mut()` yields `&mut T`
        *val_ref += 10; // Dereference `val_ref` to modify the original value
    }
    println!("Modified mutable_array: {:?}", mutable_array);

    println!("Iterating over owned array (via .into_iter()):");
    let arr_owned = [100, 200, 300]; // For `Copy` types like `i32`, values are copied.
    for val in arr_owned.into_iter() {
        // `into_iter()` yields owned `T`
        print!("{} ", val);
    }
    println!();
    // println!("Original array after .into_iter(): {:?}", arr_owned); // This would be an error if `arr_owned` contained non-Copy types, as it would be moved.

    // e. Mutability
    // Arrays are mutable if declared with `mut`. Individual elements can be modified.

    // f. Cloning (for `Copy` types, it's a simple bitwise copy)
    let copied_array = fixed_array; // For `i32` (Copy type), this is a deep copy
    println!("Copied array: {:?}", copied_array);

    // g. Returns from Functions (Copy Semantics for the array itself)
    fn array_return(arr: [i32; 3]) -> [i32; 3] {
        // `arr` is copied into the function
        [arr[0] + 1, arr[1] + 1, arr[2] + 1]
    }
    let original_arr = [1, 2, 3];
    let new_arr = array_return(original_arr);
    println!("Original array after function call: {:?}", original_arr); // Still valid
    println!("New array from function: {:?}", new_arr);

    // h. Testing for Empty (only truly empty if declared with size 0)
    let empty_array: [i32; 0] = [];
    println!("Is empty_array empty? {}", empty_array.is_empty()); // true
    println!("Is fixed_array empty? {}", fixed_array.is_empty()); // false

    // -------------------------------------------------------------------------
    // 2. Tuples (`(T1, T2, ...)`) - Fixed-size, Heterogeneous, Stack-allocated
    // -------------------------------------------------------------------------
    // Tuples are fixed-size groupings of values that can be of *different types*.
    // Their size is known at compile time and they are typically stack-allocated.
    // Useful for grouping related, but distinct, pieces of data.

    println!("\n--- 2. Tuples (`(T1, T2, ...)`) ---");

    // a. Creating Tuples
    let person_data = ("Alice", 30, true); // Tuple with string slice, integer, boolean
    let coordinates = (10.5, 20.0); // Tuple with two floats
    println!("Person data: {:?}", person_data);
    println!("Coordinates: {:?}", coordinates);

    // b. Accessing Values (by index)
    println!("Name: {}, Age: {}", person_data.0, person_data.1);

    // c. Unpacking or Destructuring
    let (name, age, is_active) = person_data;
    println!(
        "Destructured: Name: {}, Age: {}, Active: {}",
        name, age, is_active
    );

    // d. Ignoring Values during destructuring
    let (_, _, active_status) = person_data; // Use `_` to ignore specific elements
    println!("Active status (ignored others): {}", active_status);

    // e. Getting Length (not a method, length is part of its type)
    // The "length" is simply the number of elements, fixed by its type signature.

    // f. Mutability
    let mut mutable_tuple = (1, String::from("hello"));
    mutable_tuple.0 = 2; // Modify the first element
    println!("Mutable tuple after modification: {:?}", mutable_tuple);

    // g. Cloning (implicit for `Copy` types, explicit for non-`Copy` components)
    let copied_tuple = (5, true); // `i32` and `bool` are Copy types
    println!("Copied tuple: {:?}", copied_tuple);

    // h. Returns from Functions (Copy/Move Semantics based on components)
    fn tuple_return(tup: (i32, String)) -> (i32, String) {
        // `tup.1` (String) is moved in and out
        (tup.0 * 2, format!("{} world", tup.1))
    }
    let original_tup = (5, String::from("hello"));
    let new_tup = tuple_return(original_tup); // `original_tup.1` (String) is moved
    // println!("Original tuple after function call: {:?}", original_tup); // Error: value moved
    println!("New tuple from function: {:?}", new_tup);

    // i. Iterating (Not directly iterable in the same way as collections)
    // Tuples do not implement the `Iterator` trait directly for their elements.
    // You typically destructure them or access elements by index.

    // -------------------------------------------------------------------------
    // 3. Vectors (`Vec<T>`) - Dynamic-size, Homogeneous, Heap-allocated
    // -------------------------------------------------------------------------
    // Vectors are growable, dynamic-sized lists of elements of the *same type*.
    // They store their data on the heap, allowing them to grow or shrink at runtime.
    // This is the most common general-purpose list type in Rust.

    println!("\n--- 3. Vectors (`Vec<T>`) ---");

    // a. Creating Vectors
    let mut my_vec: Vec<i32> = Vec::new(); // Empty vector, explicit type
    let initial_vec = vec![10, 20, 30]; // Vector with initial values, type inferred
    println!("Empty vector: {:?}", my_vec);
    println!("Initial vector: {:?}", initial_vec);

    // b. Adding Elements (Pushing)
    my_vec.push(5);
    my_vec.push(6);
    my_vec.push(7);
    println!("Vector after pushes: {:?}", my_vec);

    // c. Accessing Values (by index and safely with `get`)
    let first_vec_val = &my_vec[0]; // Panics if index out of bounds (unsafe read)
    println!("First element (unsafe access): {}", first_vec_val);

    match my_vec.get(1) {
        // Returns `Option<&T>`, safe for out-of-bounds (safe read)
        Some(val) => println!("Second element (safe access): {}", val),
        None => println!("No second element found."),
    }

    // d. Getting Length
    println!("Length of my_vec: {}", my_vec.len());

    // e. Iterating over Vectors
    // Vectors implement `IntoIterator` for `&Vec<T>`, `&mut Vec<T>`, and `Vec<T>` (by value).
    // This allows for flexible iteration patterns.

    println!("Iterating over my_vec (immutable references via .iter()):");
    for val in my_vec.iter() {
        print!("{} ", val);
    }
    println!();

    println!("Iterating and modifying mutable_vec (mutable references via .iter_mut()):");
    let mut mutable_vec = vec![1, 2, 3];
    for val in mutable_vec.iter_mut() {
        *val *= 2; // Double each value
    }
    println!("Modified mutable_vec: {:?}", mutable_vec);

    println!("Iterating over owned vector (via .into_iter()):");
    let owned_strings_vec = vec![String::from("alpha"), String::from("beta")];
    for s in owned_strings_vec.into_iter() {
        // `s` is the owned String, `owned_strings_vec` is consumed
        println!("Owned string: {}", s);
    }
    // println!("Original owned_strings_vec: {:?}", owned_strings_vec); // Error: value moved

    // f. Mutability
    // Vectors are mutable if declared with `mut`. Elements can be modified, added, or removed.

    // g. Removing Elements (Pop, Clear, Remove)
    let mut stack_like_vec = vec![100, 200, 300];
    println!("Stack-like vector: {:?}", stack_like_vec);
    let popped_item = stack_like_vec.pop(); // Removes and returns the last element (Option<T>)
    println!("Popped item: {:?}", popped_item);
    println!("Vector after pop: {:?}", stack_like_vec);

    let removed_at_index = stack_like_vec.remove(0); // Removes element at index, shifts others
    println!("Removed item at index 0: {}", removed_at_index);
    println!("Vector after remove: {:?}", stack_like_vec);

    my_vec.clear(); // Removes all elements
    println!("Vector after clear: {:?}", my_vec);

    // h. Cloning
    let original_vec = vec![1, 2, 3];
    let cloned_vec = original_vec.clone(); // Creates a deep copy on the heap
    println!("Original vector: {:?}", original_vec);
    println!("Cloned vector: {:?}", cloned_vec);

    // i. Appending (extending with another iterable)
    let mut vec_a = vec![1, 2];
    let vec_b = vec![3, 4];
    vec_a.extend(vec_b); // Appends elements from vec_b to vec_a (vec_b is consumed)
    println!("Vector after extend: {:?}", vec_a);
    // println!("vec_b: {:?}", vec_b); // Error: value moved

    // j. Moving from/to Functions (Ownership Semantics)
    fn process_vec_ownership(v: Vec<String>) {
        // Takes ownership of the vector
        for s in v {
            println!("Processing (owned): {}", s);
        }
    } // `v` is dropped here

    let my_strings_for_move = vec![String::from("apple"), String::from("banana")];
    process_vec_ownership(my_strings_for_move); // `my_strings_for_move` is moved
    // println!("my_strings_for_move: {:?}", my_strings_for_move); // Error: value borrowed here after move

    fn process_vec_borrow(v: &Vec<i32>) {
        // Borrows the vector immutably
        println!("Processing (borrowed): {:?}", v);
    }
    let my_numbers_for_borrow = vec![10, 20];
    process_vec_borrow(&my_numbers_for_borrow);
    println!(
        "my_numbers_for_borrow still valid: {:?}",
        my_numbers_for_borrow
    );

    // k. Testing for Containing Values or Empty
    let search_vec = vec![10, 20, 30, 40];
    println!("Does search_vec contain 20? {}", search_vec.contains(&20));
    println!("Is search_vec empty? {}", search_vec.is_empty());

    // l. Inserting Custom Structs
    #[derive(Debug)] // Required for printing with `{:?}`
    struct Item {
        id: u32,
        name: String,
    }
    let mut items_vec: Vec<Item> = Vec::new();
    items_vec.push(Item {
        id: 1,
        name: String::from("Book"),
    });
    items_vec.push(Item {
        id: 2,
        name: String::from("Pen"),
    });
    println!("Vector of structs: {:?}", items_vec); // Prints the Debug representation

    // -------------------------------------------------------------------------
    // 4. Strings (`String` and `&str`) - Dynamic-size, Text, Heap-allocated
    // -------------------------------------------------------------------------
    // `String` is a growable, heap-allocated, owned, UTF-8 encoded string.
    // `&str` is a string slice, an immutable reference to a `String` or a string literal.
    // Rust's string handling is designed for correctness with Unicode.

    println!("\n--- 4. Strings (`String` and `&str`) ---");

    // a. Creating Strings
    let mut greeting = String::new(); // Empty mutable String
    let literal_string = String::from("Hello"); // From a string literal
    let converted_literal = "World".to_string(); // Convert string literal to String
    println!("Greeting: '{}'", greeting);
    println!("Literal string: '{}'", literal_string);
    println!("Converted literal: '{}'", converted_literal);

    // b. Appending
    greeting.push_str("Greetings"); // Append a string slice
    greeting.push('!'); // Append a single character
    println!("After push_str and push: '{}'", greeting);

    // c. Concatenation (`+` operator and `format!`)
    let s_part1 = String::from("Rust");
    let s_part2 = String::from("ace");
    // `+` operator takes ownership of LHS, borrows RHS.
    let full_word = s_part1 + &s_part2; // s_part1 is moved
    println!("Concatenated with +: '{}'", full_word);
    // println!("s_part1: {}", s_part1); // Error: value moved

    // `format!` macro for non-owning concatenation (recommended)
    let msg1 = String::from("Learn");
    let msg2 = String::from("Rust");
    let full_message = format!("{} {}", msg1, msg2); // msg1, msg2 are not moved
    println!("Formatted message: '{}'", full_message);
    println!("msg1 still valid: {}", msg1); // msg1 is still valid

    // d. Getting Length (bytes vs. characters)
    let unicode_str = String::from("Здравствуйте"); // Russian "Hello"
    println!("Length of '{}' (bytes): {}", unicode_str, unicode_str.len()); // Number of bytes
    println!(
        "Length of '{}' (characters): {}",
        unicode_str,
        unicode_str.chars().count() // Counts Unicode scalar values
    );

    // e. Iterating (characters, bytes)
    println!("Characters in 'Rust':");
    for c in "Rust".chars() {
        // Iterates over Unicode scalar values
        print!("{} ", c);
    }
    println!();
    println!("Bytes in 'Rust':");
    for b in "Rust".bytes() {
        // Iterates over raw UTF-8 bytes
        print!("{} ", b);
    }
    println!();

    // f. Slicing (`&str`)
    let sentence = String::from("Rust programming is fun.");
    let slice_word = &sentence[5..16]; // Slice from byte index 5 to 16 (exclusive)
    println!("Sliced word: '{}'", slice_word);
    // Note: Slicing must be on valid UTF-8 character boundaries.
    // `&sentence[0..1]` would panic for "Здравствуйте" because 'З' is 2 bytes.

    // g. Mutability
    // `String` is mutable if declared with `mut`. `&str` is immutable.

    // h. Cloning
    let original_string = String::from("clone me");
    let cloned_string = original_string.clone();
    println!("Original string: {}", original_string);
    println!("Cloned string: {}", cloned_string);

    // i. Testing for Empty
    let empty_s = String::new();
    println!("Is empty_s empty? {}", empty_s.is_empty());

    // j. Moving from/to Functions
    fn take_string_ownership(s: String) {
        println!("Function received: {}", s);
    } // `s` is dropped here

    let my_data_string = String::from("data");
    take_string_ownership(my_data_string); // `my_data_string` is moved
    // println!("my_data_string: {}", my_data_string); // Error: value moved

    fn borrow_string(s: &str) {
        // Borrows a string slice
        println!("Function borrowed: {}", s);
    }
    let my_literal = "literal data";
    borrow_string(my_literal); // `my_literal` is borrowed, still valid
    println!("my_literal still valid: {}", my_literal);

    // -------------------------------------------------------------------------
    // 5. Hash Maps (`HashMap<K, V>`) - Dynamic-size, Key-Value, Heap-allocated
    // -------------------------------------------------------------------------
    // A `HashMap<K, V>` stores mappings from unique keys of type `K` to values of type `V`.
    // Keys must be hashable (`Hash` trait) and comparable (`Eq` trait).
    // Data is stored on the heap, providing efficient average-case performance
    // for lookup, insertion, and deletion.

    println!("\n--- 5. Hash Maps (`HashMap<K, V>`) ---");

    // a. Creating Hash Maps
    let mut user_ages: HashMap<String, u32> = HashMap::new();
    println!("Empty hash map: {:?}", user_ages);

    // b. Inserting Key-Value Pairs
    user_ages.insert(String::from("Alice"), 30);
    user_ages.insert(String::from("Bob"), 25);
    println!("Hash map after inserts: {:?}", user_ages);

    // c. Accessing Values (safely with `get`, unsafely with `[]`)
    let alice_age = user_ages.get(&String::from("Alice")); // Returns `Option<&V>` (safe read)
    match alice_age {
        Some(age) => println!("Alice's age (safe): {}", age),
        None => println!("Alice not found."),
    }

    // Unsafely reading values: Panics if key is not found
    // let charlie_age = user_ages[&String::from("Charlie")]; // This would panic!
    // println!("Charlie's age (unsafe): {}", charlie_age);

    // d. Checking Existence of Keys
    println!(
        "Does map contain 'Bob'? {}",
        user_ages.contains_key(&String::from("Bob"))
    );
    println!(
        "Does map contain 'Charlie'? {}",
        user_ages.contains_key(&String::from("Charlie"))
    );

    // e. Removing Keys and Values
    let removed_bob_age = user_ages.remove(&String::from("Bob")); // Returns `Option<V>`
    println!("Removed Bob's age: {:?}", removed_bob_age);
    println!("Map after removing Bob: {:?}", user_ages);

    user_ages.clear(); // Removes all entries
    println!("Map after clear: {:?}", user_ages);

    // f. Iterating over Hash Maps
    user_ages.insert(String::from("David"), 40);
    user_ages.insert(String::from("Eve"), 35);
    println!("Iterating over user_ages (key-value pairs via .iter()):");
    for (name, age) in &user_ages {
        // Iterates over immutable references to key-value pairs
        println!("{}: {}", name, age);
    }
    println!("Iterating over keys (.keys()):");
    for key in user_ages.keys() {
        print!("{} ", key);
    }
    println!();
    println!("Iterating over values (.values()):");
    for value in user_ages.values() {
        print!("{} ", value);
    }
    println!();
    println!("Iterating over owned pairs (.into_iter()):");
    let owned_user_ages = user_ages.clone(); // Clone to demonstrate consumption
    for (name, age) in owned_user_ages.into_iter() {
        println!("Owned {}: {}", name, age);
    }
    // println!("Original owned_user_ages: {:?}", owned_user_ages); // Error: value moved

    // g. Mutability
    // HashMaps are mutable if declared with `mut`. Keys and values can be added,
    // updated, or removed.

    // h. Cloning
    let original_map = HashMap::from([(String::from("A"), 1), (String::from("B"), 2)]);
    let cloned_map = original_map.clone(); // Deep copy
    println!("Original map: {:?}", original_map);
    println!("Cloned map: {:?}", cloned_map);

    // i. Retrieving Entry and Inserting if Key is Absent (`entry().or_insert()`)
    // This is a very common and efficient way to handle "upsert" logic (update or insert).
    let mut word_counts: HashMap<String, u32> = HashMap::new();
    let sentence = "the quick brown fox jumps over the lazy dog the quick";

    for word in sentence.split_whitespace() {
        // `entry()` returns an `Entry` enum.
        // `or_insert(value)` returns a mutable reference to the value.
        // If the key exists, it returns a mutable reference to its existing value.
        // If the key doesn't exist, it inserts the provided value and returns a mutable reference to the new value.
        let count = word_counts.entry(word.to_string()).or_insert(0);
        *count += 1; // Dereference the mutable reference to increment the count
    }
    println!("Word counts: {:?}", word_counts);

    // j. Inserting Custom Structs
    #[derive(Debug)] // Required for printing with `{:?}`
    struct Order {
        order_id: u32,
        customer_name: String,
        total_amount: f64,
    }
    let mut orders: HashMap<u32, Order> = HashMap::new();
    orders.insert(
        1,
        Order {
            order_id: 1,
            customer_name: String::from("Bob"),
            total_amount: 150.75,
        },
    );
    orders.insert(
        2,
        Order {
            order_id: 2,
            customer_name: String::from("Charlie"),
            total_amount: 200.00,
        },
    );
    println!(
        "Order with ID 1: Customer '{}', Total ${}",
        orders.get(&1).unwrap().customer_name, // `.unwrap()` is unsafe here if key might be absent
        orders.get(&1).unwrap().total_amount
    );

    // k. Testing for Empty
    println!("Is user_ages map empty? {}", user_ages.is_empty());

    // -------------------------------------------------------------------------
    // 6. Iterators: General Concepts & Advanced Usage
    // -------------------------------------------------------------------------
    // Iterators in Rust are a powerful and efficient way to process sequences
    // of elements. They provide a standardized interface for traversing data
    // structures. Iterators are *lazy*, meaning they don't do any work until
    // you explicitly consume them (e.g., with a `for` loop or `collect()`).

    println!("\n--- 6. Iterators: General Concepts & Advanced Usage ---");

    // a. The `Iterator` Trait
    // At the heart of Rust's iteration system is the `Iterator` trait.
    // It defines a single required method: `next()`.
    // The `next()` method returns an `Option<Self::Item>`, where `Self::Item`
    // is the type of the items the iterator produces. It returns `Some(item)`
    // when there are more items, and `None` when the iteration is complete.

    // Example of a custom iterator (for conceptual understanding)
    struct Counter {
        count: u32,
    }

    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }

    impl Iterator for Counter {
        type Item = u32; // This iterator will produce `u32` items.

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 3 {
                // Limiting to 3 for brevity
                self.count += 1;
                Some(self.count) // Return the next item
            } else {
                None // No more items
            }
        }
    }

    println!("Custom Iterator Example:");
    let mut counter = Counter::new();
    println!("Next: {:?}", counter.next()); // Some(1)
    println!("Next: {:?}", counter.next()); // Some(2)
    for i in counter {
        // The for loop consumes the rest of the iterator
        println!("Remaining: {}", i); // 3
    }

    // b. Iterator Consumption: Single Use
    // An iterator can generally be consumed only once. Once `next()` returns `None`,
    // the iterator is exhausted. If you need to iterate multiple times over the
    // *same sequence*, you must create a new iterator (e.g., by calling `.iter()`
    // again on the original collection) or clone the collection if `into_iter()`
    // consumed it.

    println!("\nIterator Consumption: Single Use vs. Re-creation");
    let mut numbers_for_iter = vec![10, 20, 30];
    let mut iter1 = numbers_for_iter.iter();
    println!("First item from iter1: {:?}", iter1.next()); // Some(10)
    println!("Second item from iter1: {:?}", iter1.next()); // Some(20)

    // iter1 is now partially consumed. You cannot restart it.
    // To iterate again from the beginning, you need a new iterator from the source:
    let mut iter2 = numbers_for_iter.iter();
    println!("First item from new iter2: {:?}", iter2.next()); // Some(10)

    // c. Common Iterator Adapters (Transforming Iterators)
    // Iterator adapters are methods that transform an iterator into another iterator.
    // They are lazy and don't perform work until the new iterator is consumed.

    let data_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    println!("\n--- Iterator Adapter: map() ---");
    // `map()`: Applies a closure to each item, producing a new iterator with transformed items.
    let doubled_data: Vec<i32> = data_vec.iter().map(|x| x * 2).collect();
    println!("Doubled data: {:?}", doubled_data);

    println!("\n--- Iterator Adapter: filter() ---");
    // `filter()`: Keeps only items for which a closure returns `true`.
    let even_data: Vec<i32> = data_vec.iter().filter(|x| *x % 2 == 0).cloned().collect();
    // `.cloned()` is often needed after `filter` if you want owned values, as `filter` yields references.
    println!("Even data: {:?}", even_data);

    println!("\n--- Iterator Adapter: zip() ---");
    // `zip()`: Combines two iterators into a new iterator of pairs. Stops when either iterator is exhausted.
    let names = vec!["Alice", "Bob", "Charlie"];
    let ages = vec![30, 25, 35];
    let name_age_pairs: Vec<(&str, &i32)> = names.iter().zip(ages.iter()).collect();
    println!("Name-Age pairs: {:?}", name_age_pairs);

    println!("\n--- Iterator Adapter: enumerate() ---");
    // `enumerate()`: Returns an iterator that yields `(index, value)` pairs.
    for (index, value) in data_vec.iter().enumerate() {
        println!("Index {}: Value {}", index, value);
    }

    println!("\n--- Iterator Adapter: skip() ---");
    // `skip(n)`: Skips the first `n` items of the iterator.
    let skipped_data: Vec<i32> = data_vec.iter().skip(5).cloned().collect();
    println!("Data after skipping 5: {:?}", skipped_data);

    println!("\n--- Iterator Adapter: take() ---");
    // `take(n)`: Takes only the first `n` items from the iterator.
    let taken_data: Vec<i32> = data_vec.iter().take(3).cloned().collect();
    println!("Data after taking 3: {:?}", taken_data);

    // d. Common Iterator Consumers (Consuming Iterators)
    // Consumers are methods that consume the iterator, performing an action
    // or producing a final value.

    println!("\n--- Iterator Consumer: collect() ---");
    // `collect()`: Gathers all items from an iterator into a new collection.
    // The target collection type must be inferable or explicitly specified.
    let words = ["hello", "world", "rust"];
    let long_words: Vec<&str> = words.iter().filter(|s| s.len() > 4).cloned().collect();
    println!("Long words: {:?}", long_words);

    println!("\n--- Iterator Consumer: sum() ---");
    // `sum()`: Calculates the sum of all items (requires `Sum` trait).
    let total_sum: i32 = data_vec.iter().sum();
    println!("Total sum of data_vec: {}", total_sum);

    println!("\n--- Iterator Consumer: count() ---");
    // `count()`: Returns the number of items in the iterator.
    let num_count = data_vec.iter().count();
    println!("Count of data_vec: {}", num_count);

    println!("\n--- Iterator Consumers: max() / min() ---");
    // `max()` / `min()`: Finds the maximum/minimum item (returns `Option<T>`).
    let max_val = data_vec.iter().max();
    let min_val = data_vec.iter().min();
    println!("Max value: {:?}", max_val);
    println!("Min value: {:?}", min_val);

    println!("\n--- Iterator Consumer: find() ---");
    // `find()`: Returns the first item that satisfies a predicate (returns `Option<&T>`).
    let first_even = data_vec.iter().find(|&&x| x % 2 == 0); // `&&x` to dereference twice
    println!("First even number: {:?}", first_even);

    println!("\n--- Iterator Consumers: all() / any() ---");
    // `all()`: Checks if *all* items satisfy a predicate.
    // `any()`: Checks if *any* item satisfies a predicate.
    let all_positive = data_vec.iter().all(|x| *x > 0);
    let any_negative = data_vec.iter().any(|x| *x < 0);
    println!("All numbers positive? {}", all_positive);
    println!("Any numbers negative? {}", any_negative);

    // e. Breaking Out of Iterations
    // You can break out of a `for` loop (which uses iterators implicitly)
    // using the `break` keyword. For more complex conditions, `find()` or
    // `take_while()` can be useful.

    println!("\n--- Breaking Out of Iterations ---");
    let search_numbers = vec![1, 5, 10, 15, 20];
    for num in &search_numbers {
        if *num > 12 {
            println!("Found number greater than 12: {}", num);
            break; // Exit the loop
        }
    }

    // Using `find()` for early exit based on a condition (as shown above)
    // Using `take_while()` to take elements as long as a condition is true
    let less_than_10: Vec<i32> = search_numbers
        .iter()
        .take_while(|&&x| x < 10)
        .cloned()
        .collect();
    println!("Numbers less than 10: {:?}", less_than_10);

    // -------------------------------------------------------------------------
    // Conclusion: Choosing the Right Data Structure & Leveraging Iterators
    // -------------------------------------------------------------------------
    // - Arrays: Fixed-size, compile-time known length, stack-allocated. Best for
    //   small, fixed collections where performance and memory layout are critical.
    // - Tuples: Fixed-size, heterogeneous grouping of values, stack-allocated.
    //   Ideal for returning multiple values from a function or temporary groupings.
    // - Vectors: Dynamic, growable, homogeneous list, heap-allocated. The go-to
    //   choice for lists where elements are added/removed at runtime.
    // - Strings: Dynamic, growable, UTF-8 encoded text, heap-allocated. Essential
    //   for mutable, owned text data. `&str` for immutable string slices.
    // - Hash Maps: Dynamic, key-value store, heap-allocated. Perfect for efficient
    //   lookup, insertion, and deletion based on keys, like dictionaries or maps.
    // Iterators are a cornerstone of idiomatic Rust, promoting functional
    // programming patterns, efficiency, and safety across all these data structures.
    // By understanding the `Iterator` trait and its rich set of adapter and consumer methods,
    // you can write concise, expressive, and high-performance code for
    // processing data sequences.
}
