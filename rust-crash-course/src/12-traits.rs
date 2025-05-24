// This file covers fundamental Rust Trait concepts, explaining their purpose,
// how to define and implement them, and how they enable polymorphism.

use std::fmt::Debug; // Needed for derivable trait example and print_summary

fn main() {
    println!("--- Rust Traits: Defining Shared Behavior ---");

    // -------------------------------------------------------------------------
    // 1. What are Traits?
    // -------------------------------------------------------------------------
    // Traits are a way to define shared behavior in Rust. They are similar to
    // interfaces in other languages (like Java or Go), or abstract base classes
    // in C++. A trait tells the Rust compiler that a type has certain functionality.
    // By using traits, you can write generic code that works with any type
    // that implements a particular trait, leading to polymorphism and code reuse.

    // -------------------------------------------------------------------------
    // 2. Defining a Trait
    // -------------------------------------------------------------------------
    // A trait is defined with the `trait` keyword, followed by the trait name,
    // and then a block containing method signatures.
    // Traits can have associated functions (without `&self`) and methods (with `&self`).

    trait Summary {
        // Method signature: types implementing Summary must provide an implementation
        // for `summarize`.
        fn summarize(&self) -> String;

        // Another method signature.
        fn author_info(&self) -> String;
    }

    // -------------------------------------------------------------------------
    // 3. Implementing a Trait for a Type
    // -------------------------------------------------------------------------
    // To use a trait's functionality with a specific type, you must implement
    // the trait for that type using the `impl` keyword.

    #[derive(Debug)] // Required for generic `print_summary`
    struct NewsArticle {
        headline: String,
        location: String,
        author: String,
        content: String,
    }

    // Implementing the `Summary` trait for `NewsArticle`
    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }

        fn author_info(&self) -> String {
            format!("Author: {}", self.author)
        }
    }

    #[derive(Debug)] // Required for generic `print_summary`
    struct Tweet {
        username: String,
        content: String,
        reply: bool,
        retweet: bool,
    }

    // Implementing the `Summary` trait for `Tweet`
    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }

        fn author_info(&self) -> String {
            format!("Tweet by @{}", self.username)
        }
    }

    println!("\n--- Trait Implementations ---");
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again triumphed over their rivals."),
    };
    println!("News Article Summary: {}", article.summarize());
    println!("News Article Author: {}", article.author_info());

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("Tweet Summary: {}", tweet.summarize());
    println!("Tweet Author: {}", tweet.author_info());

    // -------------------------------------------------------------------------
    // 4. Default Implementations
    // -------------------------------------------------------------------------
    // Traits can provide default implementations for some or all of their methods.
    // A type can then choose to use the default implementation or override it.

    trait LoudSummary {
        fn summarize(&self) -> String; // This method must be implemented

        // Default implementation for `loud_summarize`
        fn loud_summarize(&self) -> String {
            format!("!!! {} !!!", self.summarize()) // Uses the `summarize` method
        }
    }

    struct VerboseNewsArticle {
        headline: String,
        location: String,
        author: String,
        content: String,
    }

    impl LoudSummary for VerboseNewsArticle {
        fn summarize(&self) -> String {
            format!(
                "Verbose: {}, from {} by {}",
                self.headline, self.location, self.author
            )
        }
        // `loud_summarize` is not overridden, so it uses the default implementation.
    }

    struct QuietTweet {
        username: String,
        content: String,
    }

    impl LoudSummary for QuietTweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
        // Override the default `loud_summarize` to be truly quiet.
        fn loud_summarize(&self) -> String {
            self.summarize() // No extra "!!!"
        }
    }

    println!("\n--- Default Implementations ---");
    let verbose_article = VerboseNewsArticle {
        headline: String::from("Market surges"),
        location: String::from("Global"),
        author: String::from("Financial Times"),
        content: String::from("Stocks rose sharply today."),
    };
    println!(
        "Verbose Article Loud Summary (Default): {}",
        verbose_article.loud_summarize()
    );

    let quiet_tweet = QuietTweet {
        username: String::from("silent_user"),
        content: String::from("This is a quiet message."),
    };
    println!(
        "Quiet Tweet Loud Summary (Overridden): {}",
        quiet_tweet.loud_summarize()
    );

    // -------------------------------------------------------------------------
    // 5. Traits as Parameters (Trait Bounds - Static Dispatch)
    // -------------------------------------------------------------------------
    // You can write functions that accept any type that implements a certain
    // trait. This is known as using "trait bounds" and enables polymorphism.
    // The compiler generates a specific version of the function for each type
    // that calls it, a process called "static dispatch," which has zero runtime cost.

    // Generic function that accepts any type `T` that implements `Summary` and `Debug`
    fn print_summary<T: Summary + Debug>(item: &T) {
        // `T: Summary + Debug` is the trait bound
        println!("\n--- Trait Bounds (Static Dispatch) ---");
        println!("Generic Summary: {}", item.summarize());
        println!("Debug representation: {:?}", item); // Uses Debug trait
    }

    print_summary(&article); // `article` is a NewsArticle, which implements Summary and Debug
    print_summary(&tweet); // `tweet` is a Tweet, which implements Summary and Debug

    // -------------------------------------------------------------------------
    // 6. Multiple Trait Bounds
    // -------------------------------------------------------------------------
    // You can specify multiple traits that a generic type must implement.
    // The `+` syntax is used for this.

    // Already demonstrated in `print_summary` (T: Summary + Debug)

    // -------------------------------------------------------------------------
    // 7. `impl Trait` Syntax (Shorthand for Trait Bounds)
    // -------------------------------------------------------------------------
    // For simpler cases, especially when dealing with a single trait bound,
    // the `impl Trait` syntax can be used in function parameters. It's syntactic
    // sugar for a generic trait bound.

    fn print_summary_shorthand(item: &impl Summary) {
        // Same as `item: &T where T: Summary`
        println!("\n--- `impl Trait` Syntax ---");
        println!("Shorthand Summary: {}", item.summarize());
    }
    print_summary_shorthand(&article);

    // -------------------------------------------------------------------------
    // 8. Returning Types that Implement Traits (`impl Trait` in Return Position)
    // -------------------------------------------------------------------------
    // You can also use `impl Trait` in the return position of a function to
    // indicate that the function returns *some* type that implements the specified trait,
    // without needing to name the concrete type. This is useful for abstracting
    // away the exact type, but it must return a single concrete type at compile time.

    fn returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("rust_lang"),
            content: String::from("Rust is a systems programming language."),
            reply: false,
            retweet: false,
        }
    }

    // This won't compile because it tries to return two different concrete types:
    /*
    fn returns_summarizable_error(switch: bool) -> impl Summary {
        if switch {
            NewsArticle {
                headline: String::from("Headline"),
                location: String::from("Location"),
                author: String::from("Author"),
                content: String::from("Content"),
            }
        } else {
            Tweet {
                username: String::from("user"),
                content: String::from("tweet content"),
                reply: false,
                retweet: false,
            }
        }
    }
    */
    println!("\n--- `impl Trait` in Return Position ---");
    let item = returns_summarizable();
    println!("Returned item summary: {}", item.summarize());

    // -------------------------------------------------------------------------
    // 9. Trait Objects (`dyn Trait` - Dynamic Dispatch)
    // -------------------------------------------------------------------------
    // While trait bounds (static dispatch) are compile-time, trait objects
    // enable runtime polymorphism (dynamic dispatch). A `Box<dyn Trait>` (or `&dyn Trait`)
    // means "a value of *any* type that implements `Trait`."
    // This allows you to store different concrete types in a single collection,
    // as long as they all implement the specified trait. This comes with a
    // small runtime cost.

    println!("\n--- Trait Objects (`dyn Trait`) ---");
    let mut items: Vec<Box<dyn Summary>> = Vec::new(); // A vector of trait objects
    items.push(Box::new(NewsArticle {
        headline: String::from("Local News"),
        location: String::from("Town"),
        author: String::from("Reporter"),
        content: String::from("Local story."),
    }));
    items.push(Box::new(Tweet {
        username: String::from("another_user"),
        content: String::from("Another interesting tweet."),
        reply: true,
        retweet: false,
    }));

    for item in items {
        println!("Trait Object Summary: {}", item.summarize());
    }

    // -------------------------------------------------------------------------
    // 10. Derivable Traits
    // -------------------------------------------------------------------------
    // Many common traits can be automatically implemented for your custom types
    // using the `#[derive]` attribute. This saves boilerplate code.
    // Examples: `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Hash`, `Default`.

    #[derive(Debug, Clone, PartialEq)] // Automatically implement Debug, Clone, PartialEq
    struct Point {
        x: i32,
        y: i32,
    }

    println!("\n--- Derivable Traits ---");
    let p1 = Point { x: 10, y: 20 };
    let p2 = p1.clone(); // Uses the derived `Clone` trait
    let p3 = Point { x: 10, y: 20 };

    println!("Point p1: {:?}", p1); // Uses the derived `Debug` trait
    println!("Point p2 (cloned): {:?}", p2);
    println!("Are p1 and p3 equal? {}", p1 == p3); // Uses the derived `PartialEq` trait
    println!("Are p1 and p2 equal? {}", p1 == p2);

    // -------------------------------------------------------------------------
    // 11. Newtype Pattern for Trait Implementations (Orphan Rule Workaround)
    // -------------------------------------------------------------------------
    // The "orphan rule" (or coherence rule) states that you can implement a trait
    // for a type only if either the trait OR the type is defined in the current crate.
    // This prevents conflicting implementations.
    // If you want to implement an external trait (e.g., `Debug` from `std`) for an
    // external type (e.g., `Vec<T>` from `std`), you can't directly.
    // The "newtype pattern" provides a workaround: wrap the external type in a new
    // tuple struct you define in your crate. Then you can implement traits for your new type.

    struct MyVec(Vec<i32>); // `MyVec` is a new type defined in this crate.

    impl Debug for MyVec {
        // Now we can implement Debug for `MyVec` because `MyVec` is local.
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "My custom Vec: {:?}", self.0) // Access the inner Vec with .0
        }
    }

    println!("\n--- Newtype Pattern ---");
    let my_vec = MyVec(vec![1, 2, 3]);
    println!("Debug output of MyVec: {:?}", my_vec); // Uses our custom Debug implementation.

    // -------------------------------------------------------------------------
    // 12. Orphan Rule (Coherence) - Explained in previous section
    // -------------------------------------------------------------------------
    // The orphan rule prevents conflicting trait implementations across different
    // crates. You cannot implement:
    // - A foreign trait for a foreign type.
    // - A foreign trait for a local type with foreign type parameters. (e.g., `impl MyTrait for Vec<String>`)
    // - A local trait for a foreign type. (e.g., `impl Summary for String`)
    // The newtype pattern is the standard way to get around this by making the type local.

    println!("\n--- End of Traits Examples ---");
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// This file covers fundamental Rust Trait concepts, explaining their purpose,
// how to define and implement them, and how they enable polymorphism.

// Import necessary traits from the standard library for common functionalities
use std::fmt::{Debug, Display, Formatter, Result as FmtResult}; // For Debug and Display traits

fn main() {
    println!("--- Rust Traits: Defining Shared Behavior ---");

    // -------------------------------------------------------------------------
    // 1. What are Traits? Is the implementation important?
    // -------------------------------------------------------------------------
    // Traits are a way to define shared behavior in Rust. They are similar to
    // interfaces in other languages (like Java or Go), or abstract base classes
    // in C++. A trait tells the Rust compiler that a type has certain functionality.
    //
    // The *specific implementation* of a trait's methods for a given type is
    // important for that concrete type's behavior. However, when working with
    // trait bounds or trait objects, the *details of the concrete implementation*
    // become less important than the *contract* defined by the trait itself.
    // This allows for abstraction and polymorphism, where you can write code
    // that operates on "anything that implements X" rather than on a specific type.

    // -------------------------------------------------------------------------
    // 2. Defining your own Trait
    // -------------------------------------------------------------------------
    // A trait is defined with the `trait` keyword, followed by the trait name,
    // and then a block containing method signatures.
    // Traits can have associated functions (without `&self`) and methods (with `&self`).

    trait Summary {
        // Method signature: types implementing Summary must provide an implementation
        // for `summarize`.
        fn summarize(&self) -> String;

        // Another method signature.
        fn author_info(&self) -> String;
    }

    // -------------------------------------------------------------------------
    // 3. Implementing a Trait for a Type
    // -------------------------------------------------------------------------
    // To use a trait's functionality with a specific type, you must implement
    // the trait for that type using the `impl` keyword.

    #[derive(Debug)] // Deriving `Debug` trait for easy printing with `{:?}`
    struct NewsArticle {
        headline: String,
        location: String,
        author: String,
        content: String,
    }

    // Implementing the `Summary` trait for `NewsArticle`
    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }

        fn author_info(&self) -> String {
            format!("Author: {}", self.author)
        }
    }

    #[derive(Debug)] // Deriving `Debug` trait
    struct Tweet {
        username: String,
        content: String,
        reply: bool,
        retweet: bool,
    }

    // Implementing the `Summary` trait for `Tweet`
    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }

        fn author_info(&self) -> String {
            format!("Tweet by @{}", self.username)
        }
    }

    println!("\n--- Trait Implementations ---");
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again triumphed over their rivals."),
    };
    println!("News Article Summary: {}", article.summarize());
    println!("News Article Author: {}", article.author_info());

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("Tweet Summary: {}", tweet.summarize());
    println!("Tweet Author: {}", tweet.author_info());

    // -------------------------------------------------------------------------
    // 4. Trait with New Function (Associated Function in a Trait)
    // 5. Implement `CanInitializeWithFullName` for `Person`
    // -------------------------------------------------------------------------
    // Traits can define associated functions (similar to static methods) that
    // don't take `self` as an argument. These are often used for constructor-like
    // patterns.

    trait CanInitializeWithFullName {
        // This is an associated function within the trait.
        // It acts as a factory/constructor for types implementing this trait.
        fn new_from_full_name(full_name: &str) -> Self;
    }

    // Let's define a new Person struct for this example.
    #[derive(Debug, PartialEq)] // Also derive Debug and PartialEq for Person
    struct Person {
        first_name: String,
        last_name: String,
    }

    impl CanInitializeWithFullName for Person {
        fn new_from_full_name(full_name: &str) -> Self {
            let parts: Vec<&str> = full_name.splitn(2, ' ').collect();
            let first = parts.get(0).unwrap_or(&"").to_string();
            let last = parts.get(1).unwrap_or(&"").to_string();
            Person {
                first_name: first,
                last_name: last,
            }
        }
    }

    println!("\n--- Trait with Associated Function (`new_from_full_name`) ---");
    let person_from_trait = Person::new_from_full_name("John Doe");
    println!("Person created via trait: {:?}", person_from_trait);

    // -------------------------------------------------------------------------
    // 6. Implement `fmt::Display` for Person & 7. Print out person with `{}`
    // -------------------------------------------------------------------------
    // The `std::fmt::Display` trait allows you to control how a type is printed
    // using the `{}` placeholder (like string interpolation).

    impl Display for Person {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            write!(f, "{} {}", self.first_name, self.last_name)
        }
    }

    println!("\n--- `fmt::Display` Implementation for Person ---");
    let person_display = Person {
        first_name: String::from("Jane"),
        last_name: String::from("Austen"),
    };
    println!("Printing Person with {{}}: {}", person_display); // Uses `Display` trait
    println!("Printing Person with {{:?}}: {:?}", person_display); // Uses `Debug` trait

    // -------------------------------------------------------------------------
    // 8. Traits as Parameters & 9. Trait Bound Syntax & 10. Conformance to Multiple Traits
    // -------------------------------------------------------------------------
    // These concepts enable generic programming by allowing functions to accept
    // any type that implements a specific trait (or multiple traits). This is
    // achieved through "trait bounds" (`<T: Trait>`).

    // Generic function that accepts any type `T` that implements `Summary` and `Debug`
    fn print_full_summary<T: Summary + Debug>(item: &T) {
        // `T: Summary + Debug` is the trait bound syntax
        println!("\n--- Trait Bounds (Static Dispatch) ---");
        println!("Full Summary: {}", item.summarize());
        println!("Author Info: {}", item.author_info());
        println!("Debug representation: {:?}", item); // Conformance to `Debug` trait
    }

    print_full_summary(&article); // `article` is a NewsArticle, which implements Summary and Debug
    print_full_summary(&tweet); // `tweet` is a Tweet, which implements Summary and Debug

    // -------------------------------------------------------------------------
    // 11. Trailing Trait Bound Using `where` Clause
    // -------------------------------------------------------------------------
    // For functions with many generic parameters or complex trait bounds,
    // a `where` clause can improve readability by moving the trait bounds
    // to after the function signature.

    fn compare_and_print<T, U>(item1: &T, item2: &U)
    where
        T: Summary + Debug, // Trait bounds for T
        U: Summary + Debug, // Trait bounds for U
    {
        println!("\n--- Trailing Trait Bounds using `where` ---");
        println!("Comparing two items:");
        println!("Item 1: {}", item1.summarize());
        println!("Item 2: {}", item2.summarize());
    }

    compare_and_print(&article, &tweet);

    // -------------------------------------------------------------------------
    // 12. Trait Bound Results Cannot Mix Multiple Types (`impl Trait` limitation)
    // -------------------------------------------------------------------------
    // As seen in the previous explanation, when using `impl Trait` in return position,
    // the function must return *one specific concrete type* at compile time,
    // even if multiple types implement the trait. This is a key limitation of `impl Trait`
    // and is different from trait objects (`dyn Trait`).
    /*
    fn returns_summarizable_error(switch: bool) -> impl Summary {
        if switch {
            NewsArticle { /* ... */ } // Returns NewsArticle
        } else {
            Tweet { /* ... */ } // Returns Tweet
        } // ERROR: `if` and `else` have incompatible types
    }
    */
    println!("\n--- `impl Trait` Return Type Limitation ---");
    println!("Functions returning `impl Trait` must return a single concrete type.");
    println!("Example commented out to prevent compile error.");

    // -------------------------------------------------------------------------
    // 13. Traits can be implemented on other traits (Supertraits)
    // 14. An example of trait on another trait (`HasFullNameTrait`)
    // -------------------------------------------------------------------------
    // You can declare that one trait is a "supertrait" of another. This means
    // that any type implementing the sub-trait must *also* implement the supertrait.
    // This allows the sub-trait to use methods from the supertrait without explicitly
    // stating the supertrait as a bound.

    trait HasName {
        fn get_name(&self) -> &str;
    }

    // HasFullNameTrait is a supertrait of HasName.
    // This means any type implementing HasFullNameTrait must also implement HasName.
    trait HasFullName: HasName {
        // `HasFullName` requires `HasName`
        fn get_full_name(&self) -> String {
            // Because HasFullName requires HasName, we can call `get_name()` here.
            format!("{} (Full Name)", self.get_name())
        }
    }

    // Implement HasName for our Person struct
    impl HasName for Person {
        fn get_name(&self) -> &str {
            &self.first_name // For simplicity, returning first name as base.
        }
    }

    // Implement HasFullName for Person. This is only possible because Person already implements HasName.
    impl HasFullName for Person {} // Using default implementation for get_full_name

    println!("\n--- Supertraits (`HasFullName` on `HasName`) ---");
    let rust_person = Person {
        first_name: String::from("Rust"),
        last_name: String::from("Developer"),
    };

    // We can call `get_name` directly because Person implements `HasName`.
    println!("Person's name (from HasName): {}", rust_person.get_name());

    // We can also call `get_full_name` because Person implements `HasFullName`,
    // which in turn requires `HasName`.
    println!(
        "Person's full name (from HasFullName): {}",
        rust_person.get_full_name()
    );

    // You can also use `HasFullName` as a trait bound for functions.
    fn describe_person_with_full_name<T: HasFullName + Debug>(p: &T) {
        println!("Describing person: {:?}", p);
        println!("Full name from trait: {}", p.get_full_name());
    }
    describe_person_with_full_name(&rust_person);

    // -------------------------------------------------------------------------
    // 15. Derivable Traits (revisited) & 16. Print as a debug value
    // -------------------------------------------------------------------------
    // As seen multiple times, `#[derive(Debug)]` automatically implements the `Debug` trait.
    // This allows you to print instances of your struct using the debug formatter `{:?}`.

    #[derive(Debug, Clone, PartialEq)] // Automatically implement Debug, Clone, PartialEq
    struct Product {
        id: u32,
        name: String,
    }

    println!("\n--- Derivable Traits and Debug Printing ---");
    let product1 = Product {
        id: 1,
        name: String::from("Laptop"),
    };
    println!("Product 1 (Debug print): {:?}", product1); // Uses the derived `Debug` trait
    let product2 = product1.clone(); // Uses the derived `Clone` trait
    println!("Product 2 (cloned): {:?}", product2);
    println!("Are product1 and product2 equal? {}", product1 == product2); // Uses `PartialEq`

    // -------------------------------------------------------------------------
    // 17. The `dyn Trait` (Trait Objects - Dynamic Dispatch) - revisited
    // -------------------------------------------------------------------------
    // Trait objects provide dynamic dispatch, allowing different concrete types
    // to be treated uniformly at runtime as long as they implement the same trait.
    println!("\n--- Trait Objects (`dyn Trait`) Revisited ---");
    let mut displayable_items: Vec<Box<dyn Display>> = Vec::new();
    displayable_items.push(Box::new(Person {
        first_name: String::from("Alice"),
        last_name: String::from("Wonderland"),
    }));
    displayable_items.push(Box::new(String::from("Hello, trait object!"))); // String implements Display

    for item in displayable_items {
        println!("Displayable item: {}", item); // Dynamically calls Display::fmt for each type
    }

    println!("\n--- End of Traits Examples ---");
}
