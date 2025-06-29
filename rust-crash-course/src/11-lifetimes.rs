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

//////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////

// This file covers fundamental Rust lifetime concepts, explaining why they
// are necessary, how they ensure memory safety, and how to use them.

// Import necessary modules for formatting for generic examples
use std::fmt::Debug;

fn main() {
    // Announce the start of the lifetime examples.
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
    // Inform the user that the dangling reference example is commented out.
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
    // Example: `fn foo(x: &i32)` is elided from `fn foo<'a>(x: &'a i32)`.
    // The `'a` in `fn foo<'a>(...)` declares a generic lifetime parameter,
    // while the `'a` in `x: &'a i32` uses that declared lifetime to specify
    // how long the reference `x` must be valid. This means whatever data `x`
    // points to must live at least as long as the lifetime `'a'`.

    // Rule 2: If there is exactly one input lifetime parameter, that lifetime
    // is assigned to all output lifetime parameters.
    // This function demonstrates Rule 2. The input `s` has a lifetime `'a'`,
    // and because it's the only input lifetime, the output `&str` also
    // implicitly gets the lifetime `'a'`.
    fn first_word_inferred(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }
    // Create a String to serve as the owner of the data `first_word_inferred` will borrow from.
    let sentence = String::from("hello world");
    // Call `first_word_inferred`, which returns a slice with an inferred lifetime.
    let word = first_word_inferred(&sentence);
    // Print the extracted word. This is safe because `word` (a reference to `sentence`)
    // does not outlive `sentence`.
    println!("\nInferred word: {}", word);

    // Rule 3: If there are multiple input lifetime parameters, but one of them
    // is `&self` or `&mut self` (a method), the lifetime of `self` is assigned
    // to all output lifetime parameters. (This will be demonstrated with the `Person` struct later).

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
    // the lifetime `'a'`, which is the intersection (the minimum) of the lifetimes of `x` and `y`.
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }

    // Print a header for the explicit lifetime annotation examples.
    println!("\n--- Explicit Lifetime Annotations ---");

    // Declare two strings with different ownership and potential lifetimes.
    let string1 = String::from("abcd");
    let string2 = "xyz"; // This is a string literal, which has a 'static lifetime.

    // Call the `longest` function, passing references to `string1` and `string2`.
    let result = longest(string1.as_str(), string2);
    // Print the result. This is safe because `string1` and `string2` both live long
    // enough for `result` to be valid within this scope.
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
    // Inform the user that the example for compile-time lifetime error is commented out.
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
    // The `'a` in `struct ImportantExcerpt<'a>` declares a generic lifetime parameter
    // for the struct. The `'a` in `part: &'a str` specifies that the `part` field
    // (a string slice reference) must live for at least that `'a` lifetime.
    #[derive(Debug)]
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    // Print a header for lifetimes in struct definitions.
    println!("\n--- Lifetimes in Struct Definitions ---");
    // Create a `String` to hold the novel content, which will be owned data.
    let novel = String::from("Call me Ishmael. Some years ago...");
    // Extract the first sentence as a string slice. This slice borrows from `novel`.
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    // Create an instance of `ImportantExcerpt`, holding a reference to `first_sentence`.
    // The lifetime of `i` will be tied to the lifetime of `first_sentence`, and transitively, `novel`.
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    // Print the `ImportantExcerpt` instance using debug formatting.
    println!("Important excerpt: {:?}", i);
    // Explanation: The `ImportantExcerpt` instance `i` cannot outlive `novel` because
    // `first_sentence` is a slice of `novel`. Rust ensures this at compile time.

    // Another example to demonstrate scope with structs:
    let book_title = String::from("Rust Programming");
    let excerpt_holder;
    {
        let chapter_intro = String::from("Introduction to borrowing.");
        // The lifetime of `excerpt_holder` must be shorter than or equal to
        // the *shortest* lifetime of the data it references.
        // Assigning `&book_title` to `part` is valid because `book_title`
        // lives for a longer scope than this inner block.
        excerpt_holder = ImportantExcerpt { part: &book_title };
    } // `chapter_intro` goes out of scope here.
    // `excerpt_holder` is still valid because it refers to `book_title`, which is still in scope.
    println!("Excerpt holder still valid: {:?}", excerpt_holder);
    // If we had assigned `part: &chapter_intro` inside the block, the line above
    // `println!("Excerpt holder still valid: {:?}", excerpt_holder);`
    // would not compile because `chapter_intro` would have been deallocated.

    // Lifetimes in Enums:
    // Like structs, if an enum variant holds a reference, that reference must
    // have its lifetime annotated. The lifetime annotation applies to the entire enum.
    #[derive(Debug)]
    enum Message<'a> {
        Text(&'a str), // This variant holds a string slice reference with lifetime 'a.
        Quit,
        Move { x: i32, y: i32 },
    }

    // Print a header for lifetimes in enums.
    println!("\n--- Lifetimes in Enums ---");
    // Create a `String` to be borrowed by the `Text` variant.
    let greeting = String::from("Hello, enum!");
    // Create a `Message::Text` variant, borrowing from `greeting`.
    let text_message = Message::Text(&greeting);
    println!("Message: {:?}", text_message);
    // Create other enum variants that don't hold references.
    let quit_message = Message::Quit;
    println!("Message: {:?}", quit_message);

    // The `text_message` instance cannot outlive `greeting`.
    // Uncommenting `drop(greeting);` here would cause a compile error if
    // `text_message` was subsequently used, as it would be pointing to freed memory.
    // drop(greeting); // This would cause a compile error if `text_message` was used afterwards.

    // -------------------------------------------------------------------------
    // 5. Lifetimes in Methods (Implementation Blocks)
    // -------------------------------------------------------------------------
    // When defining methods on structs that contain references, you'll also
    // use lifetime annotations. Remember Lifetime Elision Rule 3 for methods!

    // Define a `Person` struct that holds string slice references.
    #[derive(Debug)]
    struct Person<'a> {
        first_name: &'a str,
        last_name: &'a str,
    }

    // Implement methods for the `Person` struct, using the same lifetime parameter `'a`.
    impl<'a> Person<'a> {
        // Constructor for `Person`. Takes references with lifetime `'a` and returns a `Person` with that lifetime.
        fn new(first: &'a str, last: &'a str) -> Self {
            Person {
                first_name: first,
                last_name: last,
            }
        }

        // Returns the first character of the first name.
        // The returned `char` is a copy, not a reference, so no lifetime annotation is needed.
        fn get_first_initial(&self) -> Option<char> {
            self.first_name.chars().next()
        }

        // Returns a reference to the `first_name` field.
        // Due to lifetime elision rule 3, `&self` effectively becomes `&'a self`,
        // and the return type also inherits this `'a` lifetime. This means the
        // returned reference lives as long as the `Person` instance itself.
        fn get_first_name_ref(&self) -> &'a str {
            self.first_name
        }

        // A method that returns a slice of either `first_name` or `last_name`.
        // The lifetime of the returned `&str` is tied to the lifetime of `self` (i.e., `'a`).
        fn full_name_part(&self) -> &'a str {
            // In a real scenario, combining slices into a new String might be more common
            // if the combined string needs to outlive `self` or its original components.
            if self.first_name.len() > 0 {
                self.first_name
            } else {
                self.last_name
            }
        }
    }

    // Print a header for lifetime examples in structs and methods.
    println!("\n--- Lifetimes in Structs and Methods (Person Example) ---");
    // Create a `String` to be borrowed by the `Person` instance.
    let name_scope = String::from("Alice");
    // Create a `Person` instance. The `first_name` field borrows from `name_scope`.
    // Thus, `name_scope` must live at least as long as `person`.
    let person = Person::new(&name_scope, "Smith");

    // Call a method that returns a copy.
    if let Some(initial) = person.get_first_initial() {
        println!("First initial: {}", initial);
    }

    // Call a method that returns a reference tied to the person's lifetime.
    let first_name_ref = person.get_first_name_ref();
    println!("First name reference: {}", first_name_ref);

    // Call another method returning a reference.
    let full_name_part = person.full_name_part();
    println!("Full name part (from method): {}", full_name_part);

    // This demonstrates the safety: `name_scope` cannot be dropped while `person`
    // (and any references derived from `person` like `first_name_ref`) is in scope.
    // Uncommenting this line would cause a compile error if `first_name_ref` was used afterwards:
    // drop(name_scope); // error[E0505]: cannot move out of `name_scope` because it is borrowed

    // -------------------------------------------------------------------------
    // 6. The `'static` Lifetime
    // -------------------------------------------------------------------------
    // The `'static` lifetime denotes that a reference can live for the **entire**
    // duration of the program. This is typically used for string literals,
    // which are embedded directly in the program's binary, and for globally
    // declared immutable data.

    // A string literal has the `'static` lifetime.
    let s: &'static str = "I have a static lifetime.";
    println!("\nStatic string: {}", s);
    // Any string literal (`"..."`) implicitly has the `'static` lifetime.

    // -------------------------------------------------------------------------
    // 7. Generic Type Parameters, Trait Bounds, and Lifetimes Together
    // -------------------------------------------------------------------------
    // You can combine generics, trait bounds, and lifetimes for flexible yet safe code.

    // Example: A function that prints a debug representation of any two references,
    // where both references must live at least as long as 'a'.
    // `T: Debug` is a trait bound, meaning `T` must implement the `Debug` trait.
    // `'a` is the lifetime parameter, ensuring both `r1` and `r2` live for at least `'a`.
    fn print_two_references<'a, T: Debug>(r1: &'a T, r2: &'a T) {
        // Print a header for generics, trait bounds, and lifetimes.
        println!("\n--- Generics, Trait Bounds, and Lifetimes ---");
        // Print the references using debug formatting.
        println!("Reference 1: {:?}", r1);
        println!("Reference 2: {:?}", r2);
    }

    // Declare two variables of different types.
    let val1 = 100;
    let val2 = "Hello, world!"; // This also has 'static lifetime.
    // Call the generic function. The lifetime `'a'` is inferred to be valid for
    // the common scope of both `val1` and `val2`.
    print_two_references(&val1, &val2);

    // -------------------------------------------------------------------------
    // 8. Lifetime Rules Summary
    // -------------------------------------------------------------------------
    // - Every reference in Rust has an associated lifetime.
    // - Lifetimes are compile-time annotations, not runtime.
    // - Rust's borrow checker uses lifetimes to prevent dangling references.
    // - You **must** annotate lifetimes explicitly when:
    //   - A function has multiple input references, and the compiler can't
    //     unambiguously determine the output reference's lifetime.
    //   - A reference is returned from a function, and its validity depends
    //     on one of the input references.
    //   - A struct or enum variant holds a reference.
    // - Lifetime names must start with an apostrophe (`'`) and are typically
    //   short, lowercase letters (e.g., `'a'`, `'b'`).
    // - The `'static` lifetime is a special lifetime that means "lives for
    //   the entire duration of the program." String literals have this lifetime.

    // Announce the completion of lifetime examples.
    println!("\n--- End of Lifetimes Examples ---");
}
