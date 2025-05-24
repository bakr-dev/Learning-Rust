// This file covers fundamental Rust lifetime concepts, explaining why they
// are necessary, how they ensure memory safety, and how to use them.

// -------------------------------------------------------------------------
// 0. The Problem Lifetimes Solve: Dangling References
// -------------------------------------------------------------------------
// Rust's core promise is memory safety without a garbage collector.
// Lifetimes are a key part of how it achieves this. They ensure that
// references (borrowed data) never outlive the data they refer to.
// Without lifetimes, you could end up with "dangling references,"
// which point to memory that has already been deallocated, leading to
// undefined behavior and security vulnerabilities.

// Example of a dangling reference in concept (this won't compile in Rust):
/*
fn dangle_example() -> &i32 { // This function conceptually tries to return a dangling reference
    let x = 5; // x is created inside this function
    &x // We try to return a reference to x
} // x goes out of scope here, its memory is deallocated. The returned reference would be dangling!
// Rust prevents this at compile time by checking lifetimes.
*/

fn main() {
    println!("--- Rust Lifetimes: Ensuring Memory Safety ---");

    // -------------------------------------------------------------------------
    // 1. What are Lifetimes?
    // -------------------------------------------------------------------------
    // Lifetimes are a way for the Rust compiler to understand the scope for
    // which a reference is valid. They are not runtime constructs; they are
    // compile-time annotations that help the borrow checker ensure references
    // don't outlive the data they point to. Most of the time, Rust can infer
    // lifetimes, but sometimes you need to annotate them explicitly.

    // -------------------------------------------------------------------------
    // 2. Lifetime Elision Rules (When Rust infers lifetimes)
    // -------------------------------------------------------------------------
    // For simple cases, Rust follows a set of predictable rules (lifetime
    // elision rules) to infer lifetimes, so you don't always need to write
    // them explicitly.

    // Rule 1: Each input lifetime parameter gets its own lifetime parameter.
    // (e.g., fn foo<'a>(x: &'a i32))

    // Rule 2: If there is exactly one input lifetime parameter, that lifetime
    // is assigned to all output lifetime parameters.
    fn first_word_inferred(s: &str) -> &str {
        // Rust infers 'a here: fn first_word_inferred<'a>(s: &'a str) -> &'a str
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }
    let sentence = String::from("hello world");
    let word = first_word_inferred(&sentence);
    println!("\nInferred word: {}", word);
    // This is safe because 'word' (reference to 'sentence') does not outlive 'sentence'.

    // Rule 3: If there are multiple input lifetime parameters, but one of them
    // is `&self` or `&mut self` (a method), the lifetime of `self` is assigned
    // to all output lifetime parameters.

    // -------------------------------------------------------------------------
    // 3. Explicit Lifetime Annotations
    // -------------------------------------------------------------------------
    // When Rust's elision rules don't apply, or when the compiler needs more
    // help to understand relationships between multiple references, you need
    // to add explicit lifetime annotations. These annotations start with an
    // apostrophe (`'`) and are typically lowercase letters (e.g., `'a`, `'b`).
    // They go after the `&` of a reference.

    // Function that takes two string slices and returns the longer one.
    // The output reference's lifetime must be tied to the *shorter* of the two
    // input lifetimes, because the returned reference can't outlive either input.
    // Here, we explicitly state that the returned reference lives at least as long as
    // the lifetime 'a, which is the intersection of the lifetimes of x and y.
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }

    println!("\n--- Explicit Lifetime Annotations ---");

    let string1 = String::from("abcd");
    let string2 = "xyz"; // This is a string literal, which has a 'static lifetime

    let result = longest(string1.as_str(), string2);
    println!("The longest string is '{}'", result);

    // Let's demonstrate a scenario where lifetimes matter for correctness:
    // The returned reference `result` cannot outlive the shortest-lived input.
    let string_scope_example: String = String::from("long string is long");
    let result_inner;
    {
        // inner scope starts
        let string_in_inner_scope = String::from("xyz");
        // The lifetime of `string_in_inner_scope` is tied to this inner block.
        // The lifetime of `string_scope_example` extends outside this block.

        // `longest` function states that its return value (result_inner)
        // lives for the duration of the *shorter* of its inputs.
        // Here, 'a will be constrained by `string_in_inner_scope`.
        result_inner = longest(&string_scope_example, &string_in_inner_scope);
        println!("Inner scope longest: {}", result_inner);
    } // `string_in_inner_scope` goes out of scope here.

    // If result_inner was derived solely from `string_in_inner_scope`,
    // trying to use it here would be a compile-time error, preventing a dangling reference.
    // In this specific case, `result_inner` could be `string_scope_example` if it was longer,
    // which would be valid. But if `string_in_inner_scope` was longer, `result_inner`
    // would point to it, and Rust would complain if we tried to use `result_inner` here.
    //
    // The `longest` function's lifetime annotation `'a` ensures that the returned
    // reference `result_inner` is only valid for the intersection of the lifetimes
    // of `string_scope_example` and `string_in_inner_scope`.
    // So, `result_inner` cannot outlive `string_in_inner_scope`.
    // If `string_in_inner_scope` was the longest, the line below would *not* compile:
    // println!("Outer scope longest after inner block: {}", result_inner); // Would be an error if string_in_inner_scope was longer.
    //
    // Let's modify the example to guarantee a compile error if the compiler were lax:
    println!("\n--- Demonstrating Lifetime Errors (Compile-time prevention) ---");
    // This code block will fail to compile as intended, demonstrating the lifetime check.
    /*
    let hello = String::from("hello");
    let result_dangling;
    {
        let world = String::from("world"); // 'world' lives only in this inner scope
        result_dangling = longest(&hello, &world); // 'result_dangling' is tied to the shorter lifetime, which is 'world'
    } // 'world' goes out of scope here, invalidating 'result_dangling' if 'world' was longer.
      // If 'world' was longer, this line would be a compile error:
      // println!("The result is {}", result_dangling); // ERROR: 'world' does not live long enough
    */
    println!("The previous example demonstrating a compile-time lifetime error is commented out.");
    println!("If `longest` returned a reference to `world` (and `world` was longer than `hello`),");
    println!("using `result_dangling` outside its scope would be a compile error.");

    // -------------------------------------------------------------------------
    // 4. Lifetime Annotations in Struct Definitions
    // -------------------------------------------------------------------------
    // If a struct holds references, you must annotate the lifetimes of those
    // references. This tells the compiler that any instance of the struct
    // cannot outlive the data that its references point to.

    #[derive(Debug)]
    struct ImportantExcerpt<'a> {
        part: &'a str, // 'part' is a reference, and it must live for at least lifetime 'a
    }

    println!("\n--- Lifetimes in Struct Definitions ---");
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("Important excerpt: {:?}", i);
    // Here, the ImportantExcerpt instance 'i' cannot outlive 'novel' because
    // 'first_sentence' is a slice of 'novel'.

    // Another example to demonstrate scope:
    let book_title = String::from("Rust Programming");
    let excerpt_holder;
    {
        let chapter_intro = String::from("Introduction to borrowing.");
        // The lifetime of `excerpt_holder` must be shorter than or equal to
        // the *shortest* lifetime of the data it references.
        // If `chapter_intro` were assigned to `part`, `excerpt_holder`
        // would not be usable outside this block.
        excerpt_holder = ImportantExcerpt { part: &book_title }; // This is valid because `book_title` lives longer
    } // `chapter_intro` goes out of scope here.
    println!("Excerpt holder still valid: {:?}", excerpt_holder);
    // If we had assigned `part: &chapter_intro`, the line above would not compile.

    // -------------------------------------------------------------------------
    // 5. The `'static` Lifetime
    // -------------------------------------------------------------------------
    // The `'static` lifetime denotes that a reference can live for the entire
    // duration of the program. This is typically used for string literals,
    // which are stored directly in the program's binary.

    let s: &'static str = "I have a static lifetime.";
    println!("\nStatic string: {}", s);
    // Any string literal (`"..."`) has the `'static` lifetime.

    // -------------------------------------------------------------------------
    // 6. Generic Type Parameters, Trait Bounds, and Lifetimes Together
    // -------------------------------------------------------------------------
    // You can combine generics, trait bounds, and lifetimes.

    // Example: A function that prints a debug representation of any two references,
    // where both references must live at least as long as 'a.
    use std::fmt::Debug;

    fn print_two_references<'a, T: Debug>(r1: &'a T, r2: &'a T) {
        println!("\n--- Generics, Trait Bounds, and Lifetimes ---");
        println!("Reference 1: {:?}", r1);
        println!("Reference 2: {:?}", r2);
    }

    let val1 = 100;
    let val2 = "Hello, world!";
    print_two_references(&val1, &val2); // The lifetime 'a is inferred to be valid for both val1 and val2's scope.

    // -------------------------------------------------------------------------
    // 7. Lifetime Rules Summary
    // -------------------------------------------------------------------------
    // - Every reference has a lifetime.
    // - You must annotate lifetimes when:
    //   - Multiple input references could cause ambiguity for the compiler.
    //   - A reference is returned from a function.
    //   - A struct holds a reference.
    // - Lifetime names must start with `'` (e.g., `'a`).
    // - The `'static` lifetime means "lives for the entire program."

    println!("\n--- End of Lifetimes Examples ---");
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// This file covers fundamental Rust lifetime concepts, explaining why they
// are necessary, how they ensure memory safety, and how to use them.

// -------------------------------------------------------------------------
// 0. The Problem Lifetimes Solve: Dangling References
// -------------------------------------------------------------------------
// Rust's core promise is memory safety without a garbage collector.
// Lifetimes are a key part of how it achieves this. They ensure that
// references (borrowed data) never outlive the data they refer to.
// Without lifetimes, you could end up with "dangling references,"
// which point to memory that has already been deallocated, leading to
// undefined behavior and security vulnerabilities.

// Sample function that won't compile due to dangling reference:
// This function attempts to return a reference to a local variable `x`,
// which would be deallocated when the function finishes. Rust's borrow
// checker, using lifetime analysis, prevents this at compile time.
/*
fn dangle() -> &i32 { // error[E0106]: missing lifetime specifier
    let x = 5; // `x` is owned by this function
    &x // `x` does not live long enough
} // `x` goes out of scope here
*/

fn main() {
    println!("--- Rust Lifetimes: Ensuring Memory Safety ---");

    // -------------------------------------------------------------------------
    // 1. What are Lifetimes? Telling Rust How Long `&str` (and other references) Lives
    // -------------------------------------------------------------------------
    // Lifetimes are a way for the Rust compiler to understand the scope for
    // which a reference is valid. They are not runtime constructs; they are
    // compile-time annotations that help the borrow checker ensure references
    // don't outlive the data they point to. Most of the time, Rust can infer
    // lifetimes, but sometimes you need to annotate them explicitly.
    // When you write `&'a str`, you are explicitly telling Rust that this string
    // slice reference lives for a duration denoted by the lifetime parameter `'a`.

    // -------------------------------------------------------------------------
    // 2. Lifetime Elision Rules (When Rust infers lifetimes)
    // -------------------------------------------------------------------------
    // For simple cases, Rust follows a set of predictable rules (lifetime
    // elision rules) to infer lifetimes, so you don't always need to write
    // them explicitly. These rules allow for more concise code.

    // Rule 1: Each input lifetime parameter gets its own lifetime parameter.
    // (e.g., fn foo(x: &i32) becomes fn foo<'a>(x: &'a i32))

    // Rule 2: If there is exactly one input lifetime parameter, that lifetime
    // is assigned to all output lifetime parameters.
    fn first_word_inferred(s: &str) -> &str {
        // Rust infers 'a here: fn first_word_inferred<'a>(s: &'a str) -> &'a str
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }
    let sentence = String::from("hello world");
    let word = first_word_inferred(&sentence);
    println!("\nInferred word: {}", word);
    // This is safe because 'word' (reference to 'sentence') does not outlive 'sentence'.

    // Rule 3: If there are multiple input lifetime parameters, but one of them
    // is `&self` or `&mut self` (a method), the lifetime of `self` is assigned
    // to all output lifetime parameters. (More on this in the `Person` example)

    // -------------------------------------------------------------------------
    // 3. Explicit Lifetime Annotations (Generic Lifetime Annotations)
    // -------------------------------------------------------------------------
    // When Rust's elision rules don't apply (e.g., multiple input references,
    // or returning a reference that could be tied to one of several inputs),
    // or when the compiler needs more help to understand relationships between
    // multiple references, you need to add explicit lifetime annotations.
    // These annotations start with an apostrophe (`'`) and are typically
    // lowercase letters (e.g., `'a`, `'b`). They go after the `&` of a reference.

    // Function that takes two string slices and returns the longer one.
    // The output reference's lifetime must be tied to the *shorter* of the two
    // input lifetimes, because the returned reference can't outlive either input.
    // Here, we explicitly state that the returned reference lives at least as long as
    // the lifetime 'a, which is the intersection of the lifetimes of x and y.
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }

    println!("\n--- Explicit Lifetime Annotations (Generic) ---");

    let string1 = String::from("abcd");
    let string2 = "xyz"; // This is a string literal, which has a 'static lifetime

    let result = longest(string1.as_str(), string2);
    println!("The longest string is '{}'", result);

    // Demonstrating a scenario where lifetimes matter for correctness:
    // The returned reference `result` cannot outlive the shortest-lived input.
    println!("\n--- Demonstrating Lifetime Errors (Compile-time prevention) ---");
    // This code block will fail to compile as intended, demonstrating the lifetime check.
    /*
    let s1 = String::from("longer string");
    let result_dangling;
    { // inner scope starts
        let s2 = String::from("short"); // 's2' lives only in this inner scope
        // The `longest` function's signature `longest<'a>(x: &'a str, y: &'a str) -> &'a str`
        // tells the compiler that the returned reference must be valid for the duration
        // of *both* 's1' and 's2'. Since 's2' has a shorter lifetime, 'result_dangling'
        // would be constrained by 's2'.
        result_dangling = longest(&s1, &s2);
        println!("Inner scope longest: {}", result_dangling);
    } // 's2' goes out of scope here.

    // If 's2' was the result, using `result_dangling` here would point to freed memory.
    // Rust detects this: error[E0597]: `s2` does not live long enough
    println!("The result is {}", result_dangling);
    */
    println!("The previous example demonstrating a compile-time lifetime error is commented out.");
    println!("It shows that if the returned reference is tied to a shorter-lived variable,");
    println!("using that reference outside its valid scope will cause a compile error.");

    // -------------------------------------------------------------------------
    // 4. Missing Lifetime Annotations in Structs (Compile Error)
    // 5. Specify Lifetime Annotations in Structs
    // -------------------------------------------------------------------------
    // If a struct holds references, you must annotate the lifetimes of those
    // references. This tells the compiler that any instance of the struct
    // cannot outlive the data that its references point to.

    // This struct definition would cause a compile-time error:
    /*
    struct BadExcerpt {
        part: &str, // error[E0106]: missing lifetime specifier
    }
    */
    // The compiler needs to know how long `part` is expected to live.

    // Correct way to specify lifetime annotations in structs:
    #[derive(Debug)]
    struct ImportantExcerpt<'a> {
        part: &'a str, // 'part' is a reference, and it must live for at least lifetime 'a
    }

    println!("\n--- Lifetimes in Struct Definitions ---");
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("Important excerpt: {:?}", i);
    // Here, the ImportantExcerpt instance 'i' cannot outlive 'novel' because
    // 'first_sentence' is a slice of 'novel'.

    // Another example to demonstrate scope:
    let book_title = String::from("Rust Programming");
    let excerpt_holder;
    {
        let chapter_intro = String::from("Introduction to borrowing.");
        // The lifetime of `excerpt_holder` must be shorter than or equal to
        // the *shortest* lifetime of the data it references.
        // If `chapter_intro` were assigned to `part`, `excerpt_holder`
        // would not be usable outside this block.
        excerpt_holder = ImportantExcerpt { part: &book_title }; // This is valid because `book_title` lives longer
    } // `chapter_intro` goes out of scope here.
    println!("Excerpt holder still valid: {:?}", excerpt_holder);
    // If we had assigned `part: &chapter_intro`, the line above would not compile.

    // -------------------------------------------------------------------------
    // 6. Define Person, Implementation for Person, Functions Returning `&str`
    // -------------------------------------------------------------------------
    // This example demonstrates how lifetimes work with structs and methods,
    // especially when methods return references to the struct's own data.

    #[derive(Debug)]
    struct Person<'a> {
        first_name: &'a str,
        last_name: &'a str,
    }

    impl<'a> Person<'a> {
        fn new(first: &'a str, last: &'a str) -> Self {
            Person {
                first_name: first,
                last_name: last,
            }
        }

        // Returns the first character of the first name.
        // The returned char is a copy, so no lifetime annotation is needed here.
        fn get_first_initial(&self) -> Option<char> {
            self.first_name.chars().next()
        }

        // Returns the full name as a string slice.
        // The lifetime of the returned `&str` is tied to the lifetime of `&self`.
        // This is due to Lifetime Elision Rule 3: `&self`'s lifetime is assigned
        // to the output lifetime. So, `&self` effectively becomes `&'a self`,
        // and the return type becomes `&'a str`.
        fn full_name(&self) -> &'a str {
            // In a real scenario, you'd likely return a String if combining slices
            // or if the combined string needs to outlive `self`.
            // For this example, we'll return a slice of the first name if possible,
            // otherwise the last name, to demonstrate returning a reference.
            // A more realistic `full_name` returning `&str` might require a buffer.
            if self.first_name.len() > 0 {
                self.first_name
            } else {
                self.last_name
            }
        }

        // A function returning a borrowed slice of one of its fields.
        // The lifetime of the returned slice is implicitly tied to the lifetime of `self`.
        fn get_first_name_ref(&self) -> &'a str {
            self.first_name
        }
    }

    println!("\n--- Lifetimes in Structs and Methods (Person Example) ---");
    let name_scope = String::from("Alice");
    let person = Person::new(&name_scope, "Smith"); // 'name_scope' must live as long as 'person'

    if let Some(initial) = person.get_first_initial() {
        println!("First initial: {}", initial);
    }

    // `full_name()` returns a reference with the same lifetime as `person`.
    let name_ref = person.full_name();
    println!("Full name: {}", name_ref);

    let first_name_ref = person.get_first_name_ref();
    println!("First name reference: {}", first_name_ref);

    // This demonstrates the safety: `name_scope` cannot be dropped while `person` (and `name_ref`) is in scope.
    // This line would cause a compile error if uncommented and `name_ref` used afterwards:
    // drop(name_scope); // error[E0505]: cannot move out of `name_scope` because it is borrowed

    // -------------------------------------------------------------------------
    // 7. Lifetimes in Enums
    // -------------------------------------------------------------------------
    // Like structs, if an enum variant holds a reference, that reference must
    // have its lifetime annotated. The lifetime annotation applies to the entire enum.

    #[derive(Debug)]
    enum Message<'a> {
        Text(&'a str), // This variant holds a string slice reference
        Quit,
        Move { x: i32, y: i32 },
    }

    println!("\n--- Lifetimes in Enums ---");
    let greeting = String::from("Hello, enum!");
    let text_message = Message::Text(&greeting);
    println!("Message: {:?}", text_message);
    let quit_message = Message::Quit;
    println!("Message: {:?}", quit_message);

    // The `text_message` cannot outlive `greeting`.
    // drop(greeting); // This would cause a compile error if `text_message` was used afterwards.

    // -------------------------------------------------------------------------
    // 8. The `'static` Lifetime
    // -------------------------------------------------------------------------
    // The `'static` lifetime denotes that a reference can live for the entire
    // duration of the program. This is typically used for string literals,
    // which are stored directly in the program's binary.

    let s: &'static str = "I have a static lifetime.";
    println!("\nStatic string: {}", s);
    // Any string literal (`"..."`) has the `'static` lifetime.

    // -------------------------------------------------------------------------
    // 9. Generic Type Parameters, Trait Bounds, and Lifetimes Together
    // -------------------------------------------------------------------------
    // You can combine generics, trait bounds, and lifetimes.

    // Example: A function that prints a debug representation of any two references,
    // where both references must live at least as long as 'a.
    use std::fmt::Debug;

    fn print_two_references<'a, T: Debug>(r1: &'a T, r2: &'a T) {
        println!("\n--- Generics, Trait Bounds, and Lifetimes ---");
        println!("Reference 1: {:?}", r1);
        println!("Reference 2: {:?}", r2);
    }

    let val1 = 100;
    let val2 = "Another string literal"; // This also has 'static lifetime
    print_two_references(&val1, &val2); // The lifetime 'a is inferred to be valid for both val1 and val2's scope.

    // -------------------------------------------------------------------------
    // 10. Lifetime Rules Summary
    // -------------------------------------------------------------------------
    // - Every reference has a lifetime.
    // - You must annotate lifetimes when:
    //   - Multiple input references could cause ambiguity for the compiler.
    //   - A reference is returned from a function and its validity depends on input references.
    //   - A struct or enum holds a reference.
    // - Lifetime names must start with `'` (e.g., `'a`).
    // - The `'static` lifetime means "lives for the entire program."

    println!("\n--- End of Lifetimes Examples ---");
}
