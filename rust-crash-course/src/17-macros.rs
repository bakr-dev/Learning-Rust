// This file covers fundamental concepts of macros in Rust, explaining their
// purpose, types (declarative and procedural), and how they enable powerful
// compile-time code generation.

// Macros are a form of metaprogramming: code that writes other code.
// They expand into more code during the compilation phase, before the
// actual compilation of the expanded code begins.

// -------------------------------------------------------------------------
// 1. Why Use Macros?
// -------------------------------------------------------------------------
// Macros are used to reduce boilerplate code, implement domain-specific
// languages (DSLs) within Rust, and provide features that cannot be
// expressed by functions alone (e.g., variable argument lists, compile-time
// code generation based on input tokens).
// Functions are evaluated at runtime, while macros are expanded at compile time.

// -------------------------------------------------------------------------
// 2. Types of Macros in Rust
// -------------------------------------------------------------------------
// Rust primarily has two types of macros:
// a. Declarative Macros (Macros by Example / `macro_rules!`)
// b. Procedural Macros (Custom `derive`, attribute-like, function-like)

// -------------------------------------------------------------------------
// 3. Declarative Macros (`macro_rules!`)
// -------------------------------------------------------------------------
// Declarative macros, often called "macros by example" or `macro_rules!` macros,
// define a macro by providing examples of how it should look and how it should
// expand. They are defined using the `macro_rules!` construct.

// They are pattern-matching based:
// - They take Rust code as input.
// - Match that input against patterns.
// - Replace the matched code with replacement code.

// Example 3.1: A simple macro for printing "Hello!"
macro_rules! hello {
    () => {
        // This is the expansion: when `hello!()` is called, it expands to this code.
        println!("Hello from a declarative macro!");
    };
}

// Example 3.2: A macro that takes an argument and prints it
macro_rules! greet {
    ($name:expr) => {
        // `$name:expr` captures an expression and binds it to the `$name` variable.
        // `expr` is a fragment specifier for expressions.
        println!("Hello, {}!", $name);
    };
}

// Example 3.3: A macro with multiple patterns (overloading)
macro_rules! print_items {
    // Pattern 1: No arguments
    () => {
        println!("No items to print.");
    };
    // Pattern 2: Single argument (an expression)
    ($item:expr) => {
        println!("Single item: {}", $item);
    };
    // Pattern 3: Multiple arguments separated by commas (repetition)
    // `$(...)` captures multiple items.
    // `+` means one or more repetitions.
    // `,` is the separator token.
    ($($item:expr),*) => {
        print!("Multiple items: ");
        $(
            // This loop expands for each captured item.
            print!("{} ", $item);
        )*
        println!();
    };
}

// Example 3.4: Macro for creating a simple HashMap (illustrates repetition and different fragment specifiers)
// This is a simplified version of `vec!` or `map!` macros.
macro_rules! create_map {
    // `$key:expr` for keys, `$value:expr` for values.
    // `$($key:expr => $value:expr),*` captures a list of key-value pairs.
    ($($key:expr => $value:expr),*) => {
        {
            // The expansion block.
            let mut map = std::collections::HashMap::new();
            $(
                // For each key-value pair, insert it into the map.
                map.insert($key, $value);
            )*
            map // The macro expands to the initialized HashMap.
        }
    };
}

// -------------------------------------------------------------------------
// 4. Procedural Macros
// -------------------------------------------------------------------------
// Procedural macros operate on a `TokenStream` (a sequence of Rust tokens)
// and produce another `TokenStream` as output. They are much more powerful
// than declarative macros because they use Rust code to parse and generate code.
// They live in their own crate type (`proc-macro` crate).

// There are three kinds of procedural macros:
// a. Custom `derive` macros: Add code to an `impl` block based on attributes on structs/enums.
//    (e.g., `#[derive(Debug, Clone)]`)
// b. Attribute-like macros: Attach arbitrary attributes to items.
//    (e.g., `#[route("/")]`)
// c. Function-like macros: Look and behave like function calls, but operate on tokens.
//    (e.g., `sql!("SELECT * FROM users")`)

// To define procedural macros, you need:
// - A `proc-macro` crate.
// - Dependencies like `syn` (for parsing Rust code) and `quote` (for generating Rust code).

// Example (Conceptual) 4.1: Custom `derive` macro for a `Builder` pattern.
// This code cannot be directly run here as it requires a separate `proc-macro` crate.
/*
// In a `my_builder_macro` crate (type: `proc-macro`):
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident; // The name of the struct (e.g., `User`)
    let fields = if let syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Named(fields), .. }) = input.data {
        fields.named
    } else {
        panic!("Builder can only be derived for structs with named fields");
    };

    // Generate fields for the builder struct (e.g., `pub name: Option<String>`)
    let builder_fields = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        quote! {
            #field_name: std::option::Option<#field_type>
        }
    });

    // Generate setter methods (e.g., `pub fn name(mut self, name: String) -> Self`)
    let setter_methods = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        quote! {
            pub fn #field_name(mut self, #field_name: #field_type) -> Self {
                self.#field_name = Some(#field_name);
                self
            }
        }
    });

    // Generate the `build` method (e.g., `pub fn build(self) -> User`)
    let build_fields = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! {
            #field_name: self.#field_name.take().ok_or(concat!(stringify!(#field_name), " is not set"))?
        }
    });

    let builder_name = syn::Ident::new(&format!("{}Builder", name), name.span());

    let expanded = quote! {
        pub struct #builder_name {
            #(#builder_fields,)*
        }

        impl #builder_name {
            #(#setter_methods)*

            pub fn build(mut self) -> std::result::Result<#name, std::boxed::Box<dyn std::error::Error>> {
                Ok(#name {
                    #(#build_fields,)*
                })
            }
        }

        impl #name {
            pub fn builder() -> #builder_name {
                #builder_name {
                    #(#fields: std::option::Option::None,)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
*/

// -------------------------------------------------------------------------
// 5. Macro Hygiene
// -------------------------------------------------------------------------
// Rust macros are hygienic, meaning they don't accidentally capture or
// clash with variables defined in the scope where they are used.
// E.g., if a macro defines a `temp` variable, it won't conflict with
// a `temp` variable already defined in the calling scope.
// This is a significant advantage over C preprocessor macros.

// Example 5.1: Macro Hygiene in action (declarative macro)
macro_rules! with_temp {
    ($val:expr) => {
        let temp = $val + 1; // `temp` here is internal to the macro expansion
        println!("Inside macro, temp: {}", temp);
    };
}

fn main() {
    println!("--- Starting Rust Macro Examples ---");

    // ---------------------------------------------------------------------
    // Using Declarative Macros
    // ---------------------------------------------------------------------
    println!("\n--- Using Declarative Macros ---");

    // Call Example 3.1
    hello!();

    // Call Example 3.2
    greet!("Alice");
    let user = "Bob";
    greet!(user); // Can take an expression

    // Call Example 3.3 with different patterns
    print_items!();
    print_items!("apple");
    print_items!("banana", 123, true, 3.14);
    print_items!(1, 2, 3);

    // Call Example 3.4
    let my_settings = create_map! {
        "theme" => "dark",
        "font_size" => "16px",
        "debug_mode" => "true"
    };
    println!("My settings: {:?}", my_settings);
    // Note: To use `HashMap`, we needed `use std::collections::HashMap;` earlier,
    // or fully qualify it as `std::collections::HashMap`.
    // The macro itself expands to `std::collections::HashMap::new()` in this case,
    // so it doesn't strictly need the `use` statement in the calling scope for this specific macro,
    // but good practice often implies using `use` statements for types you intend to use.

    // ---------------------------------------------------------------------
    // Macro Hygiene Demonstration
    // ---------------------------------------------------------------------
    println!("\n--- Macro Hygiene Demonstration ---");
    let temp = 100; // This `temp` is in the `main` function's scope.
    println!("Before macro, main's temp: {}", temp);
    with_temp!(5); // The macro uses its own `temp` variable.
    println!("After macro, main's temp: {}", temp); // `main`'s `temp` is unaffected.

    // ---------------------------------------------------------------------
    // Using Procedural Macros (Conceptual Usage)
    // ---------------------------------------------------------------------
    println!("\n--- Using Procedural Macros (Conceptual) ---");
    println!("Procedural macros require a separate `proc-macro` crate setup.");
    println!("Demonstrating conceptual usage with a `#[derive(Builder)]` example:");

    // This part assumes a `my_builder_macro` crate exists and is linked.
    /*
    #[derive(Debug, Builder)]
    struct User {
        name: String,
        age: u8,
        email: String,
    }

    let user_instance = User::builder()
        .name("John Doe".to_string())
        .age(30)
        .email("john.doe@example.com".to_string())
        .build();

    match user_instance {
        Ok(user) => println!("Built User: {:?}", user),
        Err(e) => eprintln!("Failed to build user: {}", e),
    }

    let incomplete_user = User::builder()
        .name("Jane Doe".to_string())
        .build(); // Missing age and email

    match incomplete_user {
        Ok(user) => println!("Built User: {:?}", user),
        Err(e) => eprintln!("Failed to build incomplete user: {}", e),
    }
    */
    println!("(Please uncomment and configure a `proc-macro` crate to run the `Builder` example.)");

    println!("\n--- All macro examples completed. ---");
}
