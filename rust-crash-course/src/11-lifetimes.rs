// This file covers fundamental Rust lifetime concepts, explaining why they
// are necessary, how they ensure memory safety, and how to use them.

// Import necessary modules for formatting for generic examples
use std::fmt::Debug;

fn main() {
    println!("--- Rust Lifetimes: Ensuring Memory Safety ---");

    // -------------------------------------------------------------------------
    // 0. The Problem Lifetimes Solve: Dangling References
    // -------------------------------------------------------------------------
    // Rust's core promise is memory safety without a garbage collector.
    // Lifetimes are a key part of how it achieves this. They ensure that
    // **references** (borrowed data) never outlive the data they refer to.
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
    println!(
        "\nRust prevents dangling references like the `dangle_example` function at compile time."
    );

    // -------------------------------------------------------------------------
    // 1. What are Lifetimes? Telling Rust How Long a Reference Lives
    // -------------------------------------------------------------------------
    // Lifetimes are a way for the Rust compiler to understand the **scope** for
    // which a reference is valid. They are not runtime constructs; they are
    // **compile-time annotations** that help the borrow checker ensure references
    // don't outlive the data they point to. Most of the time, Rust can infer
    // lifetimes, but sometimes you need to annotate them explicitly.
    // When you write `&'a str`, you are explicitly telling Rust that this string
    // slice reference lives for a duration denoted by the lifetime parameter `'a'`.

    // -------------------------------------------------------------------------
    // 2. Lifetime Elision Rules (When Rust Infers Lifetimes)
    // -------------------------------------------------------------------------
    // For simple cases, Rust follows a set of predictable rules (known as
    // **lifetime elision rules**) to infer lifetimes, so you don't always need to write
    // them explicitly. These rules allow for more concise code.

    // Rule 1: Each input lifetime parameter gets its own lifetime parameter.
    // (e.g., `fn foo(x: &i32)` becomes `fn foo<'a>(x: &'a i32)`)

    // In Rust, the 'a in fn foo<'a>(...) declares a generic lifetime parameter,
    // while the 'a in x: &'a i32 uses that declared lifetime to specify how long the reference x must be valid.

    // Using Lifetimes

    // When you write x: &'a i32, you're telling the Rust compiler:

    //     "The parameter x is a reference to an i32."
    //     "This reference x must be valid for the lifetime 'a."

    // This means that whatever data x points to must live at least as long as the lifetime 'a.
    // The compiler uses this information to ensure that x doesn't become a "dangling pointer"
    // or – a reference to memory that has already been deallocated.

    // Rule 2: If there is exactly one input lifetime parameter, that lifetime
    // is assigned to all output lifetime parameters.
    fn first_word_inferred(s: &str) -> &str {
        // Rust infers 'a here as:
        // fn first_word_inferred<'a>(s: &'a str) -> &'a str
        //  <'a>: Declares a generic lifetime parameter 'a.
        //  s: &'a str: Says the input reference s lives for lifetime 'a.
        //  -> &'a str: Says the output reference also lives for lifetime 'a.
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
    // This is safe because 'word' (a reference to 'sentence') does not outlive 'sentence'.

    // Rule 3: If there are multiple input lifetime parameters, but one of them
    // is `&self` or `&mut self` (a method), the lifetime of `self` is assigned
    // to all output lifetime parameters. (We'll see this with the `Person` struct)

    // -------------------------------------------------------------------------
    // 3. Explicit Lifetime Annotations
    // -------------------------------------------------------------------------
    // When Rust's elision rules don't apply (e.g., multiple input references,
    // or returning a reference that could be tied to one of several inputs),
    // or when the compiler needs more help to understand relationships between
    // multiple references, you need to add **explicit lifetime annotations**.
    // These annotations start with an apostrophe (`'`) and are typically
    // lowercase letters (e.g., `'a'`, `'b'`). They go after the `&` of a reference.

    // Function that takes two string slices and returns the longer one.
    // The output reference's lifetime must be tied to the *shorter* of the two
    // input lifetimes, because the returned reference can't outlive either input.
    // Here, we explicitly state that the returned reference lives at least as long as
    // the lifetime `'a'`, which is the intersection of the lifetimes of `x` and `y`.
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }

    println!("\n--- Explicit Lifetime Annotations ---");

    let string1 = String::from("abcd");
    let string2 = "xyz"; // This is a string literal, which has a 'static lifetime

    let result = longest(string1.as_str(), string2);
    println!("The longest string is '{}'", result);

    // Demonstrating a scenario where lifetimes matter for correctness and Rust prevents errors:
    println!("\n--- Demonstrating Lifetime Errors (Compile-time Prevention) ---");
    // This commented-out code block below would fail to compile as intended,
    // demonstrating Rust's strict lifetime checks.
    /*
    let s1 = String::from("longer string");
    let result_dangling;
    { // inner scope starts
        let s2 = String::from("short"); // 's2' lives only in this inner scope
        // The `longest` function's signature `longest<'a>(x: &'a str, y: &'a str) -> &'a str`
        // tells the compiler that the returned reference must be valid for the duration
        // of *both* 's1' and 's2'. Since 's2' has a shorter lifetime, `result_dangling`
        // would be constrained by 's2'.
        result_dangling = longest(&s1, &s2);
        println!("Inner scope longest: {}", result_dangling);
    } // 's2' goes out of scope here, invalidating `result_dangling` if `s2` was the chosen string.

    // If 's2' was the result, using `result_dangling` here would point to freed memory.
    // Rust detects this: error[E0597]: `s2` does not live long enough
    println!("The result is {}", result_dangling);
    */
    println!("The previous example demonstrating a compile-time lifetime error is commented out.");
    println!("It shows that if the returned reference is tied to a shorter-lived variable,");
    println!("using that reference outside its valid scope will cause a compile error.");

    // -------------------------------------------------------------------------
    // 4. Lifetime Annotations in Struct and Enum Definitions
    // -------------------------------------------------------------------------
    // If a **struct** or **enum variant** holds references, you must annotate the
    // lifetimes of those references. This tells the compiler that any instance
    // of the struct/enum cannot outlive the data that its references point to.

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

    // Lifetimes in Enums:
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
    // 5. Lifetimes in Methods (Implementation Blocks)
    // -------------------------------------------------------------------------
    // When defining methods on structs that contain references, you'll also
    // use lifetime annotations. Remember Lifetime Elision Rule 3 for methods!

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

        // Returns a reference tied to the lifetime of `self`.
        // Due to lifetime elision rule 3, `&self` effectively becomes `&'a self`,
        // and the return type also inherits this `'a` lifetime.
        fn get_first_name_ref(&self) -> &'a str {
            self.first_name
        }

        // A slightly more complex method returning a reference that still adheres to 'a.
        fn full_name_part(&self) -> &'a str {
            // In a real scenario, you'd likely return a String if combining slices
            // or if the combined string needs to outlive `self`.
            // For this example, we'll return a slice of the first name if possible,
            // otherwise the last name, to demonstrate returning a reference.
            if self.first_name.len() > 0 {
                self.first_name
            } else {
                self.last_name
            }
        }
    }

    println!("\n--- Lifetimes in Structs and Methods (Person Example) ---");
    let name_scope = String::from("Alice");
    let person = Person::new(&name_scope, "Smith"); // 'name_scope' must live as long as 'person'

    if let Some(initial) = person.get_first_initial() {
        println!("First initial: {}", initial);
    }

    let first_name_ref = person.get_first_name_ref();
    println!("First name reference: {}", first_name_ref);

    let full_name_part = person.full_name_part();
    println!("Full name part (from method): {}", full_name_part);

    // This demonstrates the safety: `name_scope` cannot be dropped while `person`
    // (and any references derived from `person` like `first_name_ref`) is in scope.
    // This line would cause a compile error if uncommented and `first_name_ref` used afterwards:
    // drop(name_scope); // error[E0505]: cannot move out of `name_scope` because it is borrowed

    // -------------------------------------------------------------------------
    // 6. The `'static` Lifetime
    // -------------------------------------------------------------------------
    // The `'static` lifetime denotes that a reference can live for the **entire**
    // duration of the program. This is typically used for string literals,
    // which are embedded directly in the program's binary.

    let s: &'static str = "I have a static lifetime.";
    println!("\nStatic string: {}", s);
    // Any string literal (`"..."`) has the `'static` lifetime.

    // -------------------------------------------------------------------------
    // 7. Generic Type Parameters, Trait Bounds, and Lifetimes Together
    // -------------------------------------------------------------------------
    // You can combine generics, trait bounds, and lifetimes for flexible yet safe code.

    // Example: A function that prints a debug representation of any two references,
    // where both references must live at least as long as 'a'.
    fn print_two_references<'a, T: Debug>(r1: &'a T, r2: &'a T) {
        println!("\n--- Generics, Trait Bounds, and Lifetimes ---");
        println!("Reference 1: {:?}", r1);
        println!("Reference 2: {:?}", r2);
    }

    let val1 = 100;
    let val2 = "Hello, world!"; // This also has 'static lifetime
    print_two_references(&val1, &val2); // The lifetime 'a' is inferred to be valid for both val1 and val2's scope.

    // -------------------------------------------------------------------------
    // 8. Lifetime Rules Summary
    // -------------------------------------------------------------------------
    // - Every reference has a lifetime.
    // - You **must** annotate lifetimes when:
    //   - Multiple input references could cause ambiguity for the compiler regarding the output lifetime.
    //   - A reference is returned from a function and its validity depends on one of the input references.
    //   - A struct or enum variant holds a reference.
    // - Lifetime names must start with `'` (e.g., `'a'`).
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

/////////////////////////////////////////////////////////////////////////////////////
/// ////////////////////////////////////////////////////////////////////////////////
// This file covers fundamental Rust lifetime concepts, explaining why they
// are necessary, how they ensure memory safety, and how to use them.

// Import necessary modules for formatting for generic examples
use std::fmt::Debug;
use std::fs; // For error handling examples
use std::io::{self, Read, Write}; // For error handling examples

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // --- Rust Error Handling: panic! and Result ---
    println!("--- Starting Rust Error Handling Examples ---");

    // 1. `panic!` for Unrecoverable Errors
    // `panic!` is used when a program encounters a serious, unrecoverable
    // error. It immediately stops execution and unwinds the stack, cleaning
    // up data. This is typically used for bugs, unexpected states, or situations
    // where there's no reasonable way to proceed.
    // Uncomment the following lines to see `panic!` in action (will crash the program).
    /*
    let x = 10;
    let y = 0;
    if y == 0 {
        panic!("Cannot divide by zero! This is an unrecoverable logic error.");
    }
    let result = x / y;
    println!("Result: {}", result);
    */
    println!("\n`panic!` examples are commented out to allow the program to continue.");
    println!("`panic!` can also be caused by out-of-bounds array access.");

    // 2. `Result` for Recoverable Errors: The `enum` for Success or Failure
    // `Result<T, E>` is an enum that represents either success (`Ok(T)`) or
    // failure (`Err(E)`). `T` is the type of the value returned on success,
    // and `E` is the type of the error returned on failure.
    // This is Rust's primary mechanism for handling recoverable errors.

    // A simple function that might fail: safe_divide
    // Returns `Ok(f64)` on successful division or `Err(String)` if division by zero occurs.
    fn safe_divide(numerator: f64, denominator: f64) -> Result<f64, String> {
        if denominator == 0.0 {
            Err(String::from("Division by zero is not allowed."))
        } else {
            Ok(numerator / denominator)
        }
    }

    // 3. Handling `Result` with `match`
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

    // 4. `unwrap()` and `expect()`: Panicking on Error (Use with Caution!)
    // `unwrap()` and `expect()` extract the `Ok` value or `panic!` if the `Result` is an `Err`.
    // `expect()` allows a custom panic message. These should generally be avoided in production
    // code unless a failure truly indicates an unrecoverable bug.
    println!("\n--- `unwrap()` and `expect()` (Use with Caution!) ---");
    println!("`unwrap()` and `expect()` examples commented out to prevent panics.");
    /*
    // This would panic if the file doesn't exist:
    let content_unwrap = fs::read_to_string("this_file_does_not_exist.txt").unwrap();
    println!("Content (unwrap): {}", content_unwrap);

    // This would panic with a custom message if the file doesn't exist:
    let content_expect = fs::read_to_string("non_existent_file.txt")
        .expect("Failed to read the file! I expected this file to exist.");
    println!("Content (expect): {}", content_expect);
    */

    // A relatively safe use of `unwrap()`: when parsing a known valid number.
    let parsed_number = "42".parse::<i32>().unwrap();
    println!("Parsed number (safe unwrap): {}", parsed_number);

    // 5. Void Result Values or Errors (`Result<(), E>` or `Result<T, ()>`)
    // `()` (the unit type) is used when a function returns no meaningful value on success
    // (`Result<(), E>`) or when the error carries no specific information (`Result<T, ()>`).
    fn create_empty_file(path: &str) -> Result<(), io::Error> {
        fs::File::create(path)?; // `?` propagates `io::Error`
        println!("Successfully created empty file: {}", path);
        Ok(()) // Indicate successful completion without a specific value
    }

    println!("\n--- Void Result Values ---");
    let file_to_create = "my_empty_file.txt";
    match create_empty_file(file_to_create) {
        Ok(_) => println!("File creation operation reported success."),
        Err(e) => eprintln!("File creation failed: {}", e),
    }
    let _ = fs::remove_file(file_to_create); // Clean up

    // 6. The `?` Operator for Error Propagation (Early Exit)
    // The `?` operator concisely propagates errors. If a `Result` is `Err`,
    // the error is immediately returned from the current function.
    // Requires the current function's return type to be compatible with the error type.
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut f = fs::File::open("username.txt")?; // Propagates `io::Error` if file open fails
        let mut contents = String::new();
        f.read_to_string(&mut contents)?; // Propagates `io::Error` if read fails
        Ok(contents) // Returns contents wrapped in `Ok`
    }

    let _ = fs::write("username.txt", "Rusty_Dev"); // Create dummy file
    println!("\n--- `?` Operator for Error Propagation ---");
    match read_username_from_file() {
        Ok(username) => println!("Username from file: {}", username),
        Err(e) => eprintln!("Error reading username: {}", e),
    }
    let _ = fs::remove_file("username.txt"); // Clean up

    println!("\n--- `?` in main (Early Exit) ---");
    let content_from_non_existent = fs::read_to_string("another_non_existent.txt");
    if content_from_non_existent.is_err() {
        eprintln!(
            "Attempted to read non-existent file in main. The `?` operator would have returned here if not handled by if/else."
        );
        // Uncommenting the next line would cause main to exit with an error
        // let _content = fs::read_to_string("another_non_existent.txt")?;
    }

    // 7. `map` and `map_err` for Transforming `Result` Values
    // `map`: Transforms the `Ok` value. If `Result` is `Err`, it's passed through unchanged.
    // `map_err`: Transforms the `Err` value. If `Result` is `Ok`, it's passed through unchanged.
    println!("\n--- `map` for Ok Values ---");
    let num_str = "123";
    let parsed_and_doubled = num_str.parse::<i32>().map(|num| num * 2);
    match parsed_and_doubled {
        Ok(val) => println!("Parsed and doubled: {}", val),
        Err(e) => eprintln!("Error parsing: {}", e),
    }
    let bad_num_str = "abc";
    let parsed_and_doubled_err = bad_num_str.parse::<i32>().map(|num| num * 2);
    match parsed_and_doubled_err {
        Ok(val) => println!("Parsed and doubled: {}", val),
        Err(e) => eprintln!("Error parsing 'abc': {}", e),
    }

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

    // 8. Custom Error Types
    // Define custom error types using enums for more specific and meaningful error information.
    #[derive(Debug)]
    enum MyError {
        NotFound,
        PermissionDenied,
        InvalidInput(String),
        Io(io::Error), // Wrap `io::Error` within our custom error
    }

    // Implement `From<io::Error>` for `MyError` to allow `?` operator to convert `io::Error` to `MyError`.
    impl From<io::Error> for MyError {
        fn from(error: io::Error) -> Self {
            MyError::Io(error)
        }
    }

    // Function performing a risky operation, returning a `Result` with our custom error type.
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
            // Example of using `?` with automatic `io::Error` to `MyError` conversion
            let file_name = format!("data_{}.txt", value);
            fs::write(&file_name, format!("Some data for {}", value))?;
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
    let _ = fs::remove_file("data_50.txt"); // Clean up

    // 9. Main Function Returning Result (`fn main() -> Result<(), E>`)
    // Allows `main` to use the `?` operator, propagating errors from utility
    // functions up to the program's entry point.
    println!("\n--- Main function returning Result ---");
    println!("This entire program's `main` function demonstrates returning `Result`.");

    // Helper function for I/O operation
    fn perform_io_operation() -> Result<(), io::Error> {
        let mut file = fs::File::create("example.txt")?;
        file.write_all(b"Hello, Rust!")?;
        println!("Successfully wrote to example.txt");
        Ok(())
    }

    perform_io_operation()?; // If this fails, `main` will return the error.
    let _ = fs::remove_file("example.txt"); // Clean up

    println!("\n--- All Error Handling examples completed successfully. ---");

    // --- Rust Lifetimes: Ensuring Memory Safety ---
    println!("\n--- Starting Rust Lifetimes Examples ---");

    // 0. The Problem Lifetimes Solve: Dangling References
    // Lifetimes ensure that references never outlive the data they refer to,
    // preventing dangling references (pointers to deallocated memory).
    // Rust's borrow checker uses lifetime analysis to prevent this at compile time.
    /*
    fn dangle_example() -> &i32 { // This would fail to compile
        let x = 5;
        &x // x would be deallocated here, making the returned reference dangling
    }
    */
    println!(
        "\nRust prevents dangling references like the `dangle_example` function at compile time."
    );

    // 1. What are Lifetimes? Telling Rust How Long a Reference Lives
    // Lifetimes are compile-time annotations that help the borrow checker
    // ensure references don't outlive their data. Most are inferred,
    // but explicit annotations (`'a`, `'b'`) are sometimes needed.
    // `&'a str` means a string slice reference valid for lifetime `'a'`.

    // 2. Lifetime Elision Rules (When Rust Infers Lifetimes)
    // Rust has rules to infer lifetimes, reducing the need for explicit annotations.
    // Rule 1: Each input reference gets its own lifetime parameter.
    // (e.g., `fn foo(x: &i32)` becomes `fn foo<'a>(x: &'a i32)`)
    // Rule 2: If one input lifetime parameter, it's assigned to all output lifetime parameters.
    fn first_word_inferred(s: &str) -> &str {
        // Rust infers `'a` here: `fn first_word_inferred<'a>(s: &'a str) -> &'a str`
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
    println!("\nInferred word (Rule 2): {}", word);

    // Rule 3: If `&self` or `&mut self` is an input, its lifetime is assigned to output.
    // (Demonstrated in `Person` struct methods later)

    // 3. Explicit Lifetime Annotations
    // Required when elision rules don't apply (e.g., multiple input references,
    // or return reference could be tied to multiple inputs), or for clarity.
    // Annotations like `'a` follow `&`.
    // Function returning the longer of two string slices. The output lifetime `'a`
    // is the intersection of `x` and `y`'s lifetimes.
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }

    println!("\n--- Explicit Lifetime Annotations ---");
    let string1 = String::from("abcd");
    let string2 = "xyz"; // Has a 'static lifetime
    let result = longest(string1.as_str(), string2);
    println!("The longest string is '{}'", result);

    println!("\n--- Demonstrating Lifetime Errors (Compile-time Prevention) ---");
    println!(
        "The following commented-out code would fail to compile, demonstrating strict lifetime checks:"
    );
    /*
    let s1 = String::from("longer string");
    let result_dangling;
    { // inner scope
        let s2 = String::from("short"); // 's2' lives only in this inner scope
        // `longest` requires 'a to be valid for both inputs.
        // `s2`'s lifetime is shorter than `s1`'s.
        result_dangling = longest(&s1, &s2); // Compiler error: `s2` does not live long enough
        println!("Inner scope longest: {}", result_dangling);
    } // 's2' goes out of scope here, invalidating `result_dangling` if `s2` was chosen.
    // println!("The result is {}", result_dangling); // Attempting to use `result_dangling` here would be unsafe.
    */
    println!("It shows that if the returned reference is tied to a shorter-lived variable,");
    println!("using that reference outside its valid scope will cause a compile error.");

    // 4. Lifetime Annotations in Struct and Enum Definitions
    // Structs or enums holding references must annotate those lifetimes to ensure
    // instances don't outlive the referenced data.
    /*
    struct BadExcerpt { // error[E0106]: missing lifetime specifier
        part: &str,
    }
    */

    // Correct struct with lifetime annotation
    #[derive(Debug)]
    struct ImportantExcerpt<'a> {
        part: &'a str, // 'part' must live for at least lifetime 'a
    }

    println!("\n--- Lifetimes in Struct Definitions ---");
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("Important excerpt: {:?}", i);

    let book_title = String::from("Rust Programming");
    let excerpt_holder;
    {
        let chapter_intro = String::from("Introduction to borrowing.");
        // Valid because `book_title` lives longer than this block.
        excerpt_holder = ImportantExcerpt { part: &book_title };
        // If `part` was `&chapter_intro`, `excerpt_holder` would not be usable outside this block.
    }
    println!("Excerpt holder still valid: {:?}", excerpt_holder);

    // Lifetimes in Enums:
    // If an enum variant holds a reference, its lifetime must be annotated.
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
    // `text_message` cannot outlive `greeting`. `drop(greeting);` would cause compile error if `text_message` used afterward.

    // 5. Lifetimes in Methods (Implementation Blocks)
    // Methods on structs containing references also use lifetime annotations,
    // often implicitly via Lifetime Elision Rule 3.
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

        // Returns a copy, so no lifetime annotation needed.
        fn get_first_initial(&self) -> Option<char> {
            self.first_name.chars().next()
        }

        // Returns a reference tied to `self`'s lifetime (`'a`).
        // Elision Rule 3 applies: `&self` implies `&'a self`, and output inherits `'a`.
        fn get_first_name_ref(&self) -> &'a str {
            self.first_name
        }

        // A method returning a reference that still adheres to 'a.
        fn full_name_part(&self) -> &'a str {
            // For a real full name, you'd likely return a String if combining slices
            if self.first_name.len() > 0 {
                self.first_name
            } else {
                self.last_name
            }
        }
    }

    println!("\n--- Lifetimes in Structs and Methods (Person Example) ---");
    let name_scope = String::from("Alice");
    let person = Person::new(&name_scope, "Smith"); // `name_scope` must live as long as `person`

    if let Some(initial) = person.get_first_initial() {
        println!("First initial: {}", initial);
    }
    let first_name_ref = person.get_first_name_ref();
    println!("First name reference: {}", first_name_ref);
    let full_name_part = person.full_name_part();
    println!("Full name part (from method): {}", full_name_part);
    // `drop(name_scope);` here would cause a compile error if `person` or `first_name_ref` were used afterwards.

    // 6. The `'static` Lifetime
    // Denotes that a reference can live for the entire program duration (e.g., string literals).
    let s: &'static str = "I have a static lifetime.";
    println!("\nStatic string: {}", s);

    // 7. Generic Type Parameters, Trait Bounds, and Lifetimes Together
    // Combining generics, trait bounds, and lifetimes for flexible and safe code.
    // Example: A function printing debug representation of two references,
    // where both references must live at least as long as `'a`.
    fn print_two_references<'a, T: Debug>(r1: &'a T, r2: &'a T) {
        println!("\n--- Generics, Trait Bounds, and Lifetimes ---");
        println!("Reference 1: {:?}", r1);
        println!("Reference 2: {:?}", r2);
    }

    let val1 = 100;
    let val2 = "Hello, world!"; // Has 'static lifetime
    print_two_references(&val1, &val2); // 'a' is inferred to be valid for both `val1` and `val2`'s scope.

    println!("\n--- End of Lifetimes Examples ---");

    Ok(())
}
