// This file provides a comprehensive guide to iterators in Rust,
// covering their fundamental concepts, the `Iterator` trait, and
// various methods for working with iterators across common data structures
// like Arrays, Tuples, Vectors, and Hash Maps.

use std::collections::HashMap; // Required for HashMap

fn main() {
    // -------------------------------------------------------------------------
    // Introduction to Iterators in Rust
    // -------------------------------------------------------------------------
    // Iterators in Rust are a powerful and efficient way to process sequences
    // of elements. They provide a standardized interface for traversing data
    // structures. Iterators are *lazy*, meaning they don't do any work until
    // you explicitly consume them (e.g., with a `for` loop or `collect()`).

    // -------------------------------------------------------------------------
    // 1. The `Iterator` Trait
    // -------------------------------------------------------------------------
    // At the heart of Rust's iteration system is the `Iterator` trait.
    // It defines a single required method: `next()`.
    // The `next()` method returns an `Option<Self::Item>`, where `Self::Item`
    // is the type of the items the iterator produces. It returns `Some(item)`
    // when there are more items, and `None` when the iteration is complete.

    // Example of a custom iterator (for conceptual understanding) :

    // Define a simple struct to hold our counter's state
    struct Counter {
        count: u32,
    }

    impl Counter {
        // A constructor function to create a new Counter instance
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }

    // Implement the `Iterator` trait for our `Counter` struct
    impl Iterator for Counter {
        // We specify that this iterator will produce `u32` type items
        type Item = u32;

        // This is the core `next` method required by the `Iterator` trait
        fn next(&mut self) -> Option<Self::Item> {
            // Check if we still have numbers to count
            if self.count < 5 {
                self.count += 1; // Increment the counter
                Some(self.count) // Wrap the current count in `Some` and return it
            } else {
                // If the count has reached 5, we return `None` to signal the end of iteration
                None
            }
        }
    }

    println!("\n--- Custom Iterator Example ---");
    let mut counter = Counter::new(); // Create a new Counter instance

    // Manually call `next()` to get individual items
    println!("Next: {:?}", counter.next()); // Output: Next: Some(1)
    println!("Next: {:?}", counter.next()); // Output: Next: Some(2)

    // A `for` loop is a common way to consume an iterator.
    // It repeatedly calls `next()` until `None` is returned.
    for i in counter {
        // This loop continues from where the manual calls left off (count is now 2)
        println!("Remaining: {}", i);
    }
    // Expected Output:
    // Remaining: 3
    // Remaining: 4
    // Remaining: 5
    // -------------------------------------------------------------------------
    // 2. Iterating Over Arrays
    // -------------------------------------------------------------------------
    // Arrays are fixed-size. You can iterate over them using `.iter()`,
    // `.iter_mut()`, or `.into_iter()`.

    println!("\n--- Iterating Over Arrays ---");
    let arr = [10, 20, 30, 40];

    // a. `iter()`: Iterates over immutable references (`&T`). Does not consume the array.
    println!("Using .iter() (immutable references):");
    for &val in arr.iter() {
        // `&val` dereferences the reference to get the value
        print!("{} ", val);
    }
    println!("\nOriginal array after .iter(): {:?}", arr); // Array is still usable

    // b. `iter_mut()`: Iterates over mutable references (`&mut T`). Does not consume the array.
    println!("Using .iter_mut() (mutable references):");
    let mut mut_arr = [1, 2, 3];
    for val_ref in mut_arr.iter_mut() {
        *val_ref *= 10; // Dereference to modify the original value
    }
    println!("Modified array after .iter_mut(): {:?}", mut_arr);

    // c. `into_iter()`: Iterates over owned values (`T`). Consumes the array.
    // For `Copy` types like `i32`, it effectively copies the values.
    // For non-`Copy` types, it moves them out.
    println!("Using .into_iter() (owned values):");
    let arr_owned = [100, 200, 300];
    for val in arr_owned.into_iter() {
        // `val` is the owned value
        print!("{} ", val);
    }
    println!();
    // println!("Original array after .into_iter(): {:?}", arr_owned); // Error if non-Copy type

    // -------------------------------------------------------------------------
    // 3. Iterating Over Tuples
    // -------------------------------------------------------------------------
    // Tuples are fixed-size and heterogeneous. They don't have a direct
    // `.iter()` method like collections. Iteration is typically done by
    // destructuring or accessing elements by index. However, if you have
    // a tuple of *arrays* or a tuple that implements `IntoIterator` (rare),
    // then you can iterate. The primary way to "iterate" a tuple is destructuring.

    println!("\n--- Iterating Over Tuples (Destructuring) ---");
    let my_tuple = (1, "hello", true);
    let (a, b, c) = my_tuple; // Destructuring is the common way to "iterate"
    println!("Tuple elements: {}, {}, {}", a, b, c);

    // -------------------------------------------------------------------------
    // 4. Iterating Over Vectors (`Vec<T>`)
    // -------------------------------------------------------------------------
    // Vectors are dynamic, homogeneous lists. Iteration works similarly to arrays,
    // but `into_iter()` truly moves ownership for non-`Copy` types.

    println!("\n--- Iterating Over Vectors ---");
    let vec = vec![10, 20, 30];
    let mut mut_vec = vec![1, 2, 3];
    let owned_vec = vec![String::from("A"), String::from("B")];

    // a. `iter()` (immutable references)
    println!("Vector .iter():");
    for val in vec.iter() {
        print!("{} ", val);
    }
    println!("\nOriginal vec after .iter(): {:?}", vec);

    // b. `iter_mut()` (mutable references)
    println!("Vector .iter_mut():");
    for val_ref in mut_vec.iter_mut() {
        *val_ref += 100;
    }
    println!("Modified mut_vec: {:?}", mut_vec);

    // c. `into_iter()` (owned values - consumes the vector)
    println!("Vector .into_iter():");
    for s in owned_vec.into_iter() {
        println!("Owned string: {}", s);
    }
    // println!("Original owned_vec: {:?}", owned_vec); // Error: value moved

    // -------------------------------------------------------------------------
    // 5. Iterating Over Strings (`String` and `&str`)
    // -------------------------------------------------------------------------
    // Strings are sequences of characters (Unicode scalar values).

    println!("\n--- Iterating Over Strings ---");
    let my_string = String::from("Hello, Rust! 👋");

    // a. `chars()`: Iterates over Unicode scalar values (characters)
    println!("Characters:");
    for c in my_string.chars() {
        print!("{} ", c);
    }
    println!();

    // b. `bytes()`: Iterates over raw UTF-8 bytes
    println!("Bytes:");
    for b in my_string.bytes() {
        print!("{} ", b);
    }
    println!();

    // -------------------------------------------------------------------------
    // 6. Iterating Over Hash Maps (`HashMap<K, V>`)
    // -------------------------------------------------------------------------
    // Hash maps store key-value pairs. Iterators allow you to traverse these pairs,
    // or just keys or values.

    println!("\n--- Iterating Over Hash Maps ---");
    let mut scores = HashMap::new();
    scores.insert(String::from("Alice"), 90);
    scores.insert(String::from("Bob"), 85);
    scores.insert(String::from("Charlie"), 92);

    // a. `iter()`: Iterates over immutable references to (key, value) pairs
    println!("HashMap .iter() (key-value pairs):");
    for (key, value) in scores.iter() {
        println!("{}: {}", key, value);
    }

    // b. `keys()`: Iterates over immutable references to keys
    println!("HashMap .keys():");
    for key in scores.keys() {
        print!("{} ", key);
    }
    println!();

    // c. `values()`: Iterates over immutable references to values
    println!("HashMap .values():");
    for value in scores.values() {
        print!("{} ", value);
    }
    println!();

    // d. `into_iter()`: Iterates over owned (key, value) pairs. Consumes the HashMap.
    println!("HashMap .into_iter() (owned pairs):");
    let owned_scores = scores.clone(); // Clone to demonstrate consumption
    for (key, value) in owned_scores.into_iter() {
        println!("Owned {}: {}", key, value);
    }
    // println!("Original scores: {:?}", owned_scores); // Error: value moved

    // -------------------------------------------------------------------------
    // 7. Common Iterator Adapters and Consumers
    // -------------------------------------------------------------------------
    // Iterator adapters are methods that transform an iterator into another iterator.
    // Consumers are methods that consume the iterator and produce a final value or side effect.

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // a. `map()`: Transforms each item in the iterator.
    println!("\n--- Iterator Adapter: map() ---");
    let doubled_numbers: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("Doubled numbers: {:?}", doubled_numbers);

    // b. `filter()`: Keeps only items that satisfy a predicate.
    println!("\n--- Iterator Adapter: filter() ---");
    let even_numbers: Vec<i32> = numbers.iter().filter(|x| *x % 2 == 0).cloned().collect();
    // `.cloned()` is used because `filter` gives `&i32`, and `collect` needs owned `i32`
    println!("Even numbers: {:?}", even_numbers);

    // c. `collect()`: Consumes the iterator and collects items into a collection.
    println!("\n--- Iterator Consumer: collect() ---");
    // Already seen in `map()` and `filter()` examples.
    let words = ["hello", "world", "rust"];
    let long_words: Vec<&str> = words.iter().filter(|s| s.len() > 4).cloned().collect();
    println!("Long words: {:?}", long_words);

    // d. `sum()`: Sums up all items (requires `Sum` trait).
    println!("\n--- Iterator Consumer: sum() ---");
    let total_sum: i32 = numbers.iter().sum();
    println!("Total sum of numbers: {}", total_sum);

    // e. `count()`: Counts the number of items.
    println!("\n--- Iterator Consumer: count() ---");
    let num_count = numbers.iter().count();
    println!("Count of numbers: {}", num_count);

    // f. `max()` / `min()`: Finds the maximum/minimum item (returns `Option<T>`).
    println!("\n--- Iterator Consumers: max() / min() ---");
    let max_val = numbers.iter().max();
    let min_val = numbers.iter().min();
    println!("Max value: {:?}", max_val);
    println!("Min value: {:?}", min_val);

    // g. `zip()`: Combines two iterators into an iterator of pairs.
    println!("\n--- Iterator Adapter: zip() ---");
    let names = vec!["Alice", "Bob", "Charlie"];
    let ages = vec![30, 25, 35];
    let name_age_pairs: Vec<(&str, &i32)> = names.iter().zip(ages.iter()).collect();
    println!("Name-Age pairs: {:?}", name_age_pairs);

    // h. `enumerate()`: Returns an iterator that yields (index, value) pairs.
    println!("\n--- Iterator Adapter: enumerate() ---");
    for (index, value) in numbers.iter().enumerate() {
        println!("Index {}: Value {}", index, value);
    }

    // i. `skip()`: Skips the first `n` items.
    println!("\n--- Iterator Adapter: skip() ---");
    let skipped_numbers: Vec<i32> = numbers.iter().skip(5).cloned().collect();
    println!("Numbers after skipping 5: {:?}", skipped_numbers);

    // j. `take()`: Takes only the first `n` items.
    println!("\n--- Iterator Adapter: take() ---");
    let taken_numbers: Vec<i32> = numbers.iter().take(3).cloned().collect();
    println!("Numbers after taking 3: {:?}", taken_numbers);

    // k. `find()`: Returns the first item that satisfies a predicate (returns `Option<&T>`).
    println!("\n--- Iterator Consumer: find() ---");
    let first_even = numbers.iter().find(|x| *x % 2 == 0);
    println!("First even number: {:?}", first_even);

    // l. `all()` / `any()`: Checks if all/any items satisfy a predicate.
    println!("\n--- Iterator Consumers: all() / any() ---");
    let all_positive = numbers.iter().all(|x| *x > 0);
    let any_negative = numbers.iter().any(|x| *x < 0);
    println!("All numbers positive? {}", all_positive);
    println!("Any numbers negative? {}", any_negative);

    // -------------------------------------------------------------------------
    // 8. Iterator Consumption: Single vs. Double
    // -------------------------------------------------------------------------
    // An iterator can generally be consumed only once. Once `next()` returns `None`,
    // the iterator is exhausted. If you need to iterate multiple times, you must
    // create a new iterator (e.g., by calling `.iter()` again on the collection)
    // or clone the collection if `into_iter()` consumed it.

    println!("\n--- Iterator Consumption ---");
    let mut consumable_vec = vec![1, 2, 3];
    let mut iter1 = consumable_vec.iter();
    println!("First item from iter1: {:?}", iter1.next()); // Some(1)
    println!("Second item from iter1: {:?}", iter1.next()); // Some(2)

    // Try to create another iterator from the *same* consumed iterator (won't work)
    // let mut iter2 = iter1; // This would just move iter1, not create a new one
    // println!("First item from iter2: {:?}", iter2.next()); // Some(3)

    // To iterate again, get a new iterator from the original collection:
    let mut iter3 = consumable_vec.iter();
    println!("First item from new iter3: {:?}", iter3.next()); // Some(1)

    // -------------------------------------------------------------------------
    // 9. Breaking Out of Iterations
    // -------------------------------------------------------------------------
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

    // Using `find()` for early exit based on a condition
    let found_item = search_numbers.iter().find(|&&x| x > 12);
    println!("Found item using find(): {:?}", found_item);

    // -------------------------------------------------------------------------
    // Conclusion
    // -------------------------------------------------------------------------
    // Iterators are a cornerstone of idiomatic Rust, promoting functional
    // programming patterns, efficiency, and safety. By understanding the
    // `Iterator` trait and its rich set of adapter and consumer methods,
    // you can write concise, expressive, and high-performance code for
    // processing data sequences.
}
