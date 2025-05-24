// This file covers fundamental Rust Package, Crate, Module, and Path concepts,
// explaining how Rust organizes code, manages dependencies, and controls visibility.
// Understanding these is crucial for building scalable and maintainable Rust applications.

// -------------------------------------------------------------------------
// 0. Why Code Organization Matters: Scalability and Maintainability
// -------------------------------------------------------------------------
// As projects grow, simply putting all code in one file becomes unmanageable.
// Rust provides a robust system to organize code into logical units, manage
// dependencies, and control what parts of your code are visible to others.
// This structure helps prevent naming conflicts, improves readability, and
// facilitates collaborative development.

fn main() {
    println!("--- Rust Code Organization: Packages, Crates, Modules, Paths ---");

    // -------------------------------------------------------------------------
    // 1. Packages: Building, Testing, and Sharing Your Code
    // -------------------------------------------------------------------------
    // A **package** is Rust's highest-level code organization unit. It's what
    // you typically create with `cargo new` and manage with the Cargo build tool.
    // A package contains:
    // - One or more **crates**.
    // - A `Cargo.toml` file that describes the package (metadata, dependencies).
    // - Source code files (`src` directory).

    println!("\n--- 1. Packages ---");
    println!("A package is a Cargo-managed unit that can contain one or more crates.");
    println!("Typically, `cargo new my_project` creates a new package.");
    println!("The `Cargo.toml` file defines the package's configuration.");

    // A package can contain:
    // - Zero or one library crate (src/lib.rs).
    // - One or more binary crates (src/main.rs, src/bin/*.rs).
    // It must contain at least one crate.

    // To see this in action, you'd typically run `cargo new my_package` and inspect the created files.
    // For this demonstration, we're inside a single binary crate within a package.

    // -------------------------------------------------------------------------
    // 2. Crates: The Compilation Unit
    // -------------------------------------------------------------------------
    // A **crate** is the smallest unit of code that the Rust compiler considers
    // when compiling. When you compile a Rust program, you're compiling a crate.
    // Crates can be either:
    // - **Binary crates**: Executable applications (e.g., `src/main.rs`).
    // - **Library crates**: Reusable code that other projects can depend on (e.g., `src/lib.rs`).
    // The name of the crate is usually derived from the package name, or specified in `Cargo.toml`.

    println!("\n--- 2. Crates ---");
    println!("A crate is the compilation unit in Rust.");
    println!("This current file is part of a binary crate (`src/main.rs`).");
    println!("Library crates (`src/lib.rs`) provide reusable code for other crates.");

    // Everything defined within a crate (functions, structs, enums, etc.) is
    // implicitly available within that crate. To make items available outside
    // the crate (for library crates), they must be declared `pub`.

    // -------------------------------------------------------------------------
    // 3. Modules: Organizing Code Within a Crate
    // -------------------------------------------------------------------------
    // **Modules** are the fundamental way to organize code *within* a crate.
    // They are used to:
    // - Group related definitions (functions, structs, enums, constants, etc.).
    // - Control the privacy/visibility of items.

    println!("\n--- 3. Modules ---");
    println!("Modules organize code within a crate and control privacy.");

    // Modules can be nested, forming a tree-like hierarchy.
    // Items are private by default. Use the `pub` keyword to make them public.

    // Example of a module:
    mod greetings {
        // Items inside a module are private by default
        fn english() {
            println!("Hello!");
        }

        // Make this function public to be accessible from outside the module
        pub fn spanish() {
            println!("¡Hola!");
        }

        pub mod formal {
            pub fn english_formal() {
                println!("Good day, sir/madam.");
            }
        }

        // Example of a private item
        fn private_helper() {
            println!("This is a private helper inside greetings.");
        }

        pub fn greet_all() {
            english(); // Private function accessible within the same module
            spanish();
            private_helper();
        }
    }

    // Accessing items in modules:
    // We can call `greetings::greet_all()` because `greet_all` is `pub`.
    greetings::greet_all();

    // We can call `greetings::spanish()` directly because it's `pub`.
    greetings::spanish();

    // We can access nested public modules:
    greetings::formal::english_formal();

    // This would be a compile-time error because `english` is private:
    // greetings::english(); // error[E0603]: function `english` is private

    // This would also be a compile-time error because `private_helper` is private:
    // greetings::private_helper(); // error[E0603]: function `private_helper` is private

    println!(
        "Note: Accessing private module items directly from outside will cause compile errors, as demonstrated in commented lines."
    );

    // Modules can also be defined in separate files.
    // If you have `mod my_module;` in `main.rs`, Rust looks for `src/my_module.rs`
    // or `src/my_module/mod.rs`.

    // Example: defining a module with a struct and its methods
    mod calculator {
        pub struct BasicCalculator {
            pub value: f64, // Public field
        }

        impl BasicCalculator {
            pub fn new(start_value: f64) -> BasicCalculator {
                BasicCalculator { value: start_value }
            }

            pub fn add(&mut self, num: f64) {
                self.value += num;
            }

            fn subtract(&mut self, num: f64) {
                // Private method
                self.value -= num;
            }

            pub fn perform_subtraction(&mut self, num: f64) {
                self.subtract(num); // Private method callable from public method
            }
        }
    }

    let mut calc = calculator::BasicCalculator::new(10.0);
    println!("Initial calculator value: {}", calc.value);
    calc.add(5.0);
    println!("Value after add: {}", calc.value);
    calc.perform_subtraction(2.0);
    println!("Value after subtract via public method: {}", calc.value);

    // This would be a compile-time error because `subtract` is private:
    // calc.subtract(1.0); // error[E0616]: method `subtract` is private

    // -------------------------------------------------------------------------
    // 4. Paths: Referring to Items in the Module Tree
    // -------------------------------------------------------------------------
    // A **path** is a way to name an item in the module tree. Paths can be:
    // - **Absolute paths**: Start from the crate root (e.g., `crate::some_module::Item`).
    // - **Relative paths**: Start from the current module (e.g., `self::some_module::Item` or `super::Item`).

    println!("\n--- 4. Paths ---");
    println!("Paths specify how to find an item in the module tree.");

    // Absolute path examples:
    crate::greetings::spanish(); // `crate` refers to the current crate's root

    // Relative path examples:
    // Inside this `main` function, which is in the crate root, `self` refers to `crate`.
    self::greetings::spanish();

    // Let's define a nested module to demonstrate `super`
    mod outer_module {
        pub fn outer_function() {
            println!("Inside outer_module::outer_function");
        }

        pub mod inner_module {
            pub fn inner_function() {
                println!("Inside outer_module::inner_module::inner_function");
                // Accessing an item in the parent module using `super`
                super::outer_function();
            }
        }
    }

    outer_module::inner_module::inner_function();

    // -------------------------------------------------------------------------
    // 5. The `use` Keyword: Bringing Paths into Scope
    // -------------------------------------------------------------------------
    // The `use` keyword creates a shortcut to a path, allowing you to refer
    // to items by a shorter name. This is often used to bring items from
    // other modules or external crates into the current scope.

    println!("\n--- 5. The `use` Keyword ---");
    println!("`use` brings paths into the current scope for easier access.");

    use crate::greetings::spanish; // Bring `spanish` function into scope
    spanish(); // Now we can call it directly

    use calculator::BasicCalculator; // Bring `BasicCalculator` struct into scope
    let mut another_calc = BasicCalculator::new(20.0);
    another_calc.add(3.0);
    println!(
        "Another calculator value (using `use`): {}",
        another_calc.value
    );

    // Renaming with `as`:
    use crate::greetings::formal::english_formal as formal_greeting;
    formal_greeting();

    // Using `*` (glob operator) to bring all public items into scope (generally discouraged)
    // use crate::greetings::*;
    // spanish(); // Would work if `greetings` was brought in with glob

    // -------------------------------------------------------------------------
    // 6. External Crates and the `extern crate` Keyword
    // -------------------------------------------------------------------------
    // To use code from an external library (a crate downloaded from crates.io),
    // you first declare it as a dependency in your `Cargo.toml`.
    // Then, in your Rust code, you can use `use` statements to bring items
    // from that crate into scope. The `extern crate` syntax is usually not
    // needed in Rust 2018 edition and later, as Cargo automatically links them.

    println!("\n--- 6. External Crates ---");
    println!("External crates are declared in Cargo.toml and typically imported with `use`.");

    // To demonstrate this, imagine we've added `rand = "0.8.5"` to `Cargo.toml`
    // under `[dependencies]`.

    // Then in the code:
    use rand::Rng; // Bring the Rng trait into scope

    let mut rng = rand::thread_rng(); // Use a function from the `rand` crate
    let random_number: u8 = rng.gen_range(1..=10);
    println!("Random number from 'rand' crate: {}", random_number);

    // No `extern crate rand;` is typically needed with Cargo in modern Rust.

    // -------------------------------------------------------------------------
    // 7. Re-exporting Names (Using `pub use`)
    // -------------------------------------------------------------------------
    // Sometimes you want to bring an item into scope and also make it public
    // for others who use your crate. This is called re-exporting.

    println!("\n--- 7. Re-exporting Names (`pub use`) ---");
    println!("`pub use` makes an item accessible through your module's path.");

    mod my_utility_module {
        pub mod string_utils {
            pub fn capitalize(s: &str) -> String {
                s.to_uppercase()
            }
        }

        // Re-export `capitalize` so users can access it directly via `my_utility_module::capitalize`
        pub use string_utils::capitalize;
    }

    // Now, `capitalize` can be accessed directly through `my_utility_module`
    let original = "hello rust";
    let capitalized = my_utility_module::capitalize(original);
    println!("Capitalized string (via re-export): {}", capitalized);

    // Without `pub use string_utils::capitalize;`, we would have to use:
    // let capitalized = my_utility_module::string_utils::capitalize(original);

    println!("\n--- End of Rust Code Organization Examples ---");
}

