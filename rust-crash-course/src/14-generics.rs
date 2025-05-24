// This file covers fundamental Rust Generics concepts, explaining their necessity,
// how they enable flexible and reusable code, and how to use them with functions,
// structs, enums, and methods.

// -------------------------------------------------------------------------
// 0. The Problem Generics Solve: Code Duplication and Rigidity
// -------------------------------------------------------------------------
// Without generics, you would often write separate functions or data structures
// for each type you want to handle, even if the logic is identical. This leads
// to significant code duplication and makes your code less flexible and harder to maintain.
// Generics allow you to write code once and have it work with many different types.

// Sample function without generics (imagine needing this for i32, f64, String, etc.):
fn find_largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn find_largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            // This requires 'char' to implement the 'PartialOrd' trait
            largest = item;
        }
    }
    largest
}

fn main() {
    println!("--- Rust Generics: Writing Flexible and Reusable Code ---");

    // Example of code duplication
    let number_list = vec![34, 50, 25, 100, 65];
    let result_num = find_largest_i32(&number_list);
    println!("\nLargest number: {}", result_num);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result_char = find_largest_char(&char_list);
    println!("Largest char: {}", result_char);

    // This demonstrates the problem generics solve: The logic for finding the
    // largest item is the same, but we had to write separate functions for `i32` and `char`.

    // -------------------------------------------------------------------------
    // 1. What are Generics? Type Parameters
    // -------------------------------------------------------------------------
    // Generics are abstract stand-ins for concrete types or other properties.
    // When writing code using generics, you specify type parameters in angle brackets,
    // `<T>`, where `T` is a placeholder for a type. When the code is compiled,
    // Rust performs "monomorphization," replacing the generic type parameters with
    // the concrete types used in your code, generating specialized versions.
    // This means there's no runtime overhead for using generics in Rust.

    // -------------------------------------------------------------------------
    // 2. Generics in Function Definitions
    // -------------------------------------------------------------------------
    // We can make the `find_largest` function generic to work with any type
    // that supports comparison (`PartialOrd`) and copying (`Copy`).

    // Here, `T` is a generic type parameter.
    // `PartialOrd` is a trait bound, meaning `T` must implement the `PartialOrd` trait
    // (for comparison like `>`).
    // `Copy` is another trait bound, meaning `T` must implement `Copy`
    // (so we can copy elements from the slice).
    fn find_largest_generic<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];
        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    println!("\n--- 2. Generics in Function Definitions ---");

    let number_list_gen = vec![34, 50, 25, 100, 65];
    let result_num_gen = find_largest_generic(&number_list_gen);
    println!("Largest number (generic): {}", result_num_gen);

    let char_list_gen = vec!['y', 'm', 'a', 'q'];
    let result_char_gen = find_largest_generic(&char_list_gen);
    println!("Largest char (generic): {}", result_char_gen);

    // Another generic function example:
    fn print_two_items<T: std::fmt::Debug>(item1: T, item2: T) {
        println!("Item 1: {:?}", item1);
        println!("Item 2: {:?}", item2);
    }

    print_two_items(10, 20);
    print_two_items("hello", "world");

    // -------------------------------------------------------------------------
    // 3. Generics in Struct Definitions
    // -------------------------------------------------------------------------
    // You can define structs to hold values of generic types. This is useful
    // for creating data structures that can work with any type.

    // Here, `X` and `Y` are generic type parameters for the struct fields.
    // They don't need trait bounds unless you perform operations on them inside the struct's methods.
    #[derive(Debug)]
    struct Point<X, Y> {
        x: X,
        y: Y,
    }

    println!("\n--- 3. Generics in Struct Definitions ---");

    let integer_point = Point { x: 5, y: 10 };
    println!("Integer point: {:?}", integer_point);

    let float_point = Point { x: 1.0, y: 4.0 };
    println!("Float point: {:?}", float_point);

    let mixed_point = Point { x: 3, y: 5.5 };
    println!("Mixed point: {:?}", mixed_point);

    // Example with a single generic type parameter:
    #[derive(Debug)]
    struct Wrapper<T> {
        value: T,
    }

    let wrapper_int = Wrapper { value: 42 };
    let wrapper_str = Wrapper { value: "hello" };
    println!("Wrapper int: {:?}", wrapper_int);
    println!("Wrapper str: {:?}", wrapper_str);

    // -------------------------------------------------------------------------
    // 4. Generics in Enum Definitions
    // -------------------------------------------------------------------------
    // Enums can also be generic, allowing their variants to hold values of generic types.
    // A common example is the `Option<T>` enum, which can hold any type or nothing.
    // Another is `Result<T, E>`, which holds either a success value of type `T` or an
    // error value of type `E`.

    println!("\n--- 4. Generics in Enum Definitions ---");

    // Standard Library Option<T> and Result<T, E> are prime examples:
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    let some_number = Some(5);
    let some_string = Some("a string");
    let no_value: Option<i32> = None; // Type annotation needed for `None`

    println!("Some number: {:?}", some_number);
    println!("Some string: {:?}", some_string);
    println!("No value: {:?}", no_value);

    let ok_result: Result<i32, String> = Ok(123);
    let err_result: Result<i32, String> = Err(String::from("Something went wrong!"));

    println!("Ok result: {:?}", ok_result);
    println!("Err result: {:?}", err_result);

    // -------------------------------------------------------------------------
    // 5. Generics in Method Definitions
    // -------------------------------------------------------------------------
    // Methods can be generic too. You can define generic parameters for the `impl` block
    // to work with generic types defined in the struct, or you can define generic
    // parameters only for specific methods.

    println!("\n--- 5. Generics in Method Definitions ---");

    // Generic parameters in the `impl` block (for the struct's generic types)
    impl<X, Y> Point<X, Y> {
        fn get_x(&self) -> &X {
            &self.x
        }

        // Method that takes a generic parameter different from the struct's generics
        fn mixup<V, W>(self, other: Point<V, W>) -> Point<X, W> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    let p1 = Point { x: 5, y: 10.4 };
    println!("p1.x: {}", p1.get_x()); // Works for any X type

    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2); // Combines x from p1 and y from p2
    println!("p3 (mixed up): {:?}", p3);

    impl<T> Wrapper<T> {
        fn unwrap_value(self) -> T {
            self.value
        }
    }

    let unwrapped_int = wrapper_int.unwrap_value();
    println!("Unwrapped int: {}", unwrapped_int);

    // -------------------------------------------------------------------------
    // 6. Performance of Code with Generics (Monomorphization)
    // -------------------------------------------------------------------------
    // Rust's generics compile to code that is just as fast as if you had written
    // a separate implementation for each concrete type. This is because of a process
    // called "monomorphization."

    println!("\n--- 6. Performance of Code with Generics (Monomorphization) ---");

    // When the compiler sees `find_largest_generic<i32>(...)` and `find_largest_generic<char>(...)`,
    // it effectively generates two separate, optimized versions of the function:
    // fn find_largest_generic_i32(list: &[i32]) -> i32 { ... }
    // fn find_largest_generic_char(list: &[char]) -> char { ... }
    // There is no runtime cost or abstraction penalty for using generics in Rust.

    // -------------------------------------------------------------------------
    // 7. Generics and Trait Bounds: Constraining Generic Types
    // -------------------------------------------------------------------------
    // Trait bounds are essential for making generics useful. They specify what
    // capabilities a generic type must have. This allows you to call methods
    // on generic types, knowing they will exist.

    println!("\n--- 7. Generics and Trait Bounds ---");

    // We already saw this with `find_largest_generic<T: PartialOrd + Copy>`.
    // Let's create another example:
    // A function that prints a debug representation of any two references,
    // where both references must live at least as long as 'a.
    use std::fmt::Debug; // Import the Debug trait

    fn print_debug_info<'a, T: Debug>(item1: &'a T, item2: &'a T) {
        println!("Item 1 (Debug): {:?}", item1);
        println!("Item 2 (Debug): {:?}", item2);
    }

    let d_val1 = 123.45;
    let d_val2 = "Another string";
    print_debug_info(&d_val1, &d_val2);

    // Using `where` clauses for cleaner trait bounds, especially with many bounds:
    fn print_multiple_bounds<T, U>(item_t: T, item_u: U)
    where
        T: Debug + Clone,       // T must implement Debug and Clone
        U: PartialEq + Default, // U must implement PartialEq and Default
    {
        println!("\n--- Using `where` clauses ---");
        println!("Item T (Debug): {:?}", item_t);
        println!("Item U (Default): {:?}", U::default()); // U::default() comes from the Default trait
        // Note: You can't directly compare item_t and item_u using PartialEq unless they are of the same type.
        // This is just to demonstrate the `where` clause syntax.
    }

    print_multiple_bounds(String::from("hello"), 5); // String implements Debug, Clone. i32 implements PartialEq, Default.

    // -------------------------------------------------------------------------
    // 8. Monomorphization vs. Dynamic Dispatch (Trait Objects)
    // -------------------------------------------------------------------------
    // While generics are monomorphized (static dispatch), Rust also has trait objects
    // (`dyn Trait`) which use dynamic dispatch. Understanding the difference is key.

    println!("\n--- 8. Monomorphization vs. Dynamic Dispatch ---");

    // Generics (Monomorphization/Static Dispatch):
    // - Code is duplicated for each concrete type at compile time.
    // - Zero runtime overhead.
    // - Type information is known at compile time.
    // - Used when you know the concrete type at compile time.

    // Trait Objects (Dynamic Dispatch - covered in more detail in Trait lesson):
    // - Allows you to refer to types that implement a trait without knowing their
    //   concrete type at compile time.
    // - Achieved using `dyn Trait`.
    // - Incurs a small runtime overhead for dynamic dispatch (vtable lookup).
    // - Size of the trait object must be known at compile time (often requires `Box<dyn Trait>`).

    trait Greeter {
        fn greet(&self);
    }

    struct EnglishGreeter;
    impl Greeter for EnglishGreeter {
        fn greet(&self) {
            println!("Hello!");
        }
    }

    struct SpanishGreeter;
    impl Greeter for SpanishGreeter {
        fn greet(&self) {
            println!("¡Hola!");
        }
    }

    // Generic function (static dispatch)
    fn call_greeter_generic<G: Greeter>(greeter: G) {
        greeter.greet();
    }

    // Function using a trait object (dynamic dispatch)
    fn call_greeter_dynamic(greeter: Box<dyn Greeter>) {
        greeter.greet();
    }

    println!("\nStatic Dispatch with Generics:");
    call_greeter_generic(EnglishGreeter);
    call_greeter_generic(SpanishGreeter);

    println!("\nDynamic Dispatch with Trait Objects:");
    call_greeter_dynamic(Box::new(EnglishGreeter));
    call_greeter_dynamic(Box::new(SpanishGreeter));

    // -------------------------------------------------------------------------
    // 9. When to Use Generics
    // -------------------------------------------------------------------------
    // - When you have functions or data structures that perform the same logic
    //   on different types, but the *exact* type is known at compile time.
    // - When you want to maximize code reuse and avoid duplication.
    // - When performance is critical and you want zero runtime overhead for abstraction.
    // - When building libraries or APIs where users need to provide their own types.

    println!("\n--- End of Generics Examples ---");
}
