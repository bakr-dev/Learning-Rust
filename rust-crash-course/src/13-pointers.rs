// This file covers fundamental Rust Pointer concepts, explaining their types,
// use cases, and how they interact with Rust's ownership and borrowing rules.

// -------------------------------------------------------------------------
// 0. What are Pointers?
// -------------------------------------------------------------------------
// In Rust, the term "pointer" generally refers to a memory address.
// While Rust's core philosophy emphasizes references (`&T` and `&mut T`)
// for safe, managed access to data, it also provides "raw pointers" (`*const T`
// and `*mut T`) for scenarios requiring low-level memory manipulation,
// often in `unsafe` blocks. References are a type of smart pointer
// managed by the borrow checker.

fn main() {
    println!("--- Rust Pointers: Understanding Memory Addresses ---");

    // -------------------------------------------------------------------------
    // 1. References: Rust's Safe Pointers (`&T` and `&mut T`)
    // -------------------------------------------------------------------------
    // References are Rust's primary way to interact with data by "borrowing" it.
    // They are guaranteed by the borrow checker to always point to valid data
    // and never cause data races. They are the most common and safest form of
    // "pointer-like" access in Rust.

    println!("\n--- 1. References: Rust's Safe Pointers ---");

    let x = 10;
    // Immutable reference: `&T`
    // You can have multiple immutable references to the same data.
    let r1 = &x;
    let r2 = &x;
    println!("Original value (x): {}", x);
    println!("Immutable reference 1 (r1): {}", r1);
    println!("Immutable reference 2 (r2): {}", r2);
    // *r1 = 20; // ERROR: cannot assign to `*r1` because it is behind a `&` reference

    let mut y = 20;
    // Mutable reference: `&mut T`
    // You can have *only one* mutable reference to a piece of data at a time
    // within a given scope. This prevents data races.
    let r_mut = &mut y;
    println!("Original mutable value (y): {}", y);
    println!("Mutable reference (r_mut): {}", r_mut);
    *r_mut = 30; // Dereferencing to modify the original value
    println!("Modified value via mutable reference (y): {}", y);
    // let another_r_mut = &mut y; // ERROR: cannot borrow `y` as mutable more than once at a time
    // println!("Another mutable reference (another_r_mut): {}", another_r_mut);
    println!(
        "Note: Attempting to create another mutable reference to 'y' would cause a compile error, as shown in the commented line."
    );

    // -------------------------------------------------------------------------
    // 2. Raw Pointers: Unsafe Low-Level Access (`*const T` and `*mut T`)
    // -------------------------------------------------------------------------
    // Raw pointers are similar to pointers in C/C++. They are not guaranteed
    // to point to valid memory, are not checked by the borrow checker for
    // validity, and do not enforce Rust's ownership rules. Dereferencing
    // raw pointers requires an `unsafe` block. They are typically used for:
    // - FFI (Foreign Function Interface) calls to C libraries.
    // - Building custom data structures that the borrow checker cannot reason about.
    // - Interacting with operating system primitives.

    println!("\n--- 2. Raw Pointers: Unsafe Low-Level Access ---");

    let num = 5;
    let r = &num; // Regular reference
    let raw_ptr_const = r as *const i32; // Coerce a reference to an immutable raw pointer

    let mut val = 100;
    let r_mut_val = &mut val; // Regular mutable reference
    let raw_ptr_mut = r_mut_val as *mut i32; // Coerce a mutable reference to a mutable raw pointer

    println!("Address of num: {:p}", r); // Print memory address of reference
    println!("Raw pointer (const): {:p}", raw_ptr_const); // Print memory address of raw pointer
    println!("Raw pointer (mut): {:p}", raw_ptr_mut);

    // Dereferencing raw pointers requires an `unsafe` block
    unsafe {
        println!("Value pointed to by raw_ptr_const: {}", *raw_ptr_const);
        *raw_ptr_mut = 200; // Modify value through mutable raw pointer
        println!("Value modified via raw_ptr_mut: {}", *raw_ptr_mut);
    }
    println!("Original 'val' after raw pointer modification: {}", val);

    // Creating raw pointers directly from addresses (extremely dangerous and rare):
    // let address = 0x0123_4567_89AB_CDEF_usize; // Example arbitrary address
    // let bad_ptr = address as *const u8;
    // unsafe {
    //     // This would likely cause a segmentation fault or undefined behavior!
    //     // println!("Value at arbitrary address: {}", *bad_ptr);
    // }
    println!(
        "Note: Creating raw pointers from arbitrary addresses is extremely dangerous and can lead to crashes. The example is commented out."
    );

    // -------------------------------------------------------------------------
    // 3. Pointer Arithmetic (Unsafe)
    // -------------------------------------------------------------------------
    // Raw pointers allow pointer arithmetic, which is also an `unsafe` operation
    // because it can easily lead to out-of-bounds memory access.

    println!("\n--- 3. Pointer Arithmetic (Unsafe) ---");

    let arr = [1, 2, 3, 4, 5];
    let ptr = arr.as_ptr(); // Get a raw pointer to the first element

    unsafe {
        // Accessing the first element
        println!("First element: {}", *ptr);

        // Moving to the second element (ptr + 1)
        let second_element_ptr = ptr.add(1);
        println!("Second element (ptr + 1): {}", *second_element_ptr);

        // Moving to the fourth element (ptr + 3)
        let fourth_element_ptr = ptr.add(3);
        println!("Fourth element (ptr + 3): {}", *fourth_element_ptr);

        // Attempting to access out of bounds (dangerous!)
        // let out_of_bounds_ptr = ptr.add(10);
        // println!("Out of bounds access: {}", *out_of_bounds_ptr); // UB
    }
    println!(
        "Note: Performing pointer arithmetic out of bounds leads to Undefined Behavior. The example is commented out."
    );

    // -------------------------------------------------------------------------
    // 4. Function Pointers or function as value (Pass functions as values ​​are done by Pointers)
    // -------------------------------------------------------------------------
    // Rust also has function pointers, which allow you to treat functions
    // as values that can be passed around and called dynamically.

    println!("\n--- 4. Function Pointers ---");

    fn add_one(i: i32) -> i32 {
        i + 1
    }

    fn apply_operation(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg)
    }

    let fn_ptr: fn(i32) -> i32 = add_one; // Type annotation is optional
    let result = fn_ptr(5);
    println!("Result of calling function pointer: {}", result);

    let result_applied = apply_operation(add_one, 10);
    println!(
        "Result of applying operation via function pointer: {}",
        result_applied
    );

    // -------------------------------------------------------------------------
    // 5. Smart Pointers (Brief Mention)
    // -------------------------------------------------------------------------
    // While not "raw" pointers, Rust provides various "smart pointers" that
    // manage ownership and provide additional capabilities beyond simple
    // references. They encapsulate data and provide various guarantees.
    // Key smart pointers include:
    // - `Box<T>`: For allocating data on the heap.
    // - `Rc<T>`: Reference counting, for multiple owners of data.
    // - `Arc<T>`: Atomic reference counting, for safe shared ownership across threads.
    // - `RefCell<T>`: Interior mutability (allowing mutable borrows through an immutable reference).
    // - `Cow<'a, T>`: Clone-on-write, for efficient handling of owned or borrowed data.

    println!("\n--- 5. Smart Pointers (Brief Mention) ---");

    // Example: Box<T> for heap allocation
    let b = Box::new(5);
    println!("Boxed integer: {}", b);

    // Example: Rc<T> for shared ownership
    use std::rc::Rc;
    let rc_data = Rc::new("shared data".to_string());
    let rc_clone1 = Rc::clone(&rc_data);
    let rc_clone2 = Rc::clone(&rc_data);
    println!(
        "Rc data: {}, {} (count: {})",
        rc_data,
        rc_clone1,
        Rc::strong_count(&rc_data)
    );

    println!("\n--- End of Pointers Examples ---");
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////////////

// This file delves into advanced Rust pointer concepts, covering the stack vs. heap,
// various smart pointers like Box, Rc, and Cell/RefCell, and their implications
// for ownership, borrowing, and mutability.

use std::cell::Cell;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};

fn main() {
    println!("--- Advanced Rust Pointers and Memory Management ---");

    // -------------------------------------------------------------------------
    // 0. Stack vs. Heap: Where Data Lives
    // -------------------------------------------------------------------------
    // Rust, like many compiled languages, manages memory using two primary regions:
    // the stack and the heap. Understanding where data lives is crucial for
    // comprehending pointers and ownership.

    println!("\n--- 0. Stack vs. Heap ---");

    // Stack:
    // - Stores values in a Last-In, First-Out (LIFO) order.
    // - Used for fixed-size data, function calls (local variables, return addresses).
    // - Very fast allocation and deallocation because it's just pushing and popping.
    // - Data is automatically dropped when its owning scope ends.
    let stack_int = 10; // `10` is directly on the stack
    let stack_bool = true; // `true` is directly on the stack
    let stack_array = [1, 2, 3]; // The entire array is on the stack if known size

    println!(
        "Stack values: {} (int), {} (bool), {:?} (array)",
        stack_int, stack_bool, stack_array
    );

    // Heap:
    // - Used for data of unknown size at compile time or data that needs to live
    //   longer than the current scope.
    // - More flexible but slower allocation/deallocation (requires finding space).
    // - Data is accessed indirectly via a pointer stored on the stack.
    // - Rust manages deallocation automatically through ownership and dropping.
    let heap_string = String::from("Hello, Heap!"); // String data is on the heap, `heap_string` (ptr, len, capacity) is on the stack
    println!("Heap string: {}", heap_string);

    // -------------------------------------------------------------------------
    // 1. Box<T>: Heap Allocation
    // -------------------------------------------------------------------------
    // `Box<T>` is a smart pointer that allows you to allocate data on the heap.
    // It's used when you have data whose size isn't known at compile time,
    // or when you want to own a value and only have a pointer to it (e.g., recursive data structures).
    // When a `Box` goes out of scope, its destructor is called, and the heap memory is freed.

    println!("\n--- 1. Box<T>: Heap Allocation ---");

    // Example of Box with de-referencing
    let b = Box::new(5); // `5` is allocated on the heap, `b` (the Box) is on the stack
    println!("Boxed value: {}", b); // `Box` implements `Display` by dereferencing
    println!("Value inside Box (dereferenced explicitly): {}", *b); // Explicit dereference

    // Deref Trait on Box:
    // `Box<T>` implements the `Deref` trait. This trait allows `Box<T>` to be treated
    // like a `&T` (a reference) when dereferenced using the `*` operator.
    // It also enables "deref coercion," where `Box<T>` can automatically convert
    // to `&T` when passed to functions expecting a reference.

    // -------------------------------------------------------------------------
    // 2. Implementing Our Own Box (Conceptual `MyBox`)
    // -------------------------------------------------------------------------
    // To understand `Box` better, let's conceptualize how we might implement a simple version.

    println!("\n--- 2. Implementing Our Own Box (Conceptual `MyBox`) ---");

    struct MyBox<T>(T); // A tuple struct holding a value of type T

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }

        // Add a new function in impl for BoxedValue
        fn describe(&self) {
            println!("This MyBox contains a value.");
        }
    }

    // Create an instance of MyBox
    let my_val = MyBox::new(String::from("Hello from MyBox!"));
    println!("MyBox instance created.");
    my_val.describe();

    // Trying to de-reference MyBox value (fail scenario initially)
    // println!("Trying to dereference (compile error initially): {}", *my_val);
    // This will fail with "the trait `Deref` is not implemented for `MyBox<String>`"

    // Implement Deref for MyBox
    impl<T> Deref for MyBox<T> {
        type Target = T; // The type that we are dereferencing to

        fn deref(&self) -> &Self::Target {
            &self.0 // Return a reference to the inner value
        }
    }

    // Now, de-reference MyBox value
    println!("Value inside MyBox (dereferenced explicitly): {}", *my_val);

    // Point to the de-referenced value: `*ptr` as short hand for `*(ptr.deref())`
    // The `*` operator is syntactic sugar. When you write `*my_val`, Rust
    // internally calls `*(my_val.deref())`. This is why implementing `Deref`
    // allows the `*` operator to work.

    // -------------------------------------------------------------------------
    // 3. Implicit Deref Coercion in Functions
    // -------------------------------------------------------------------------
    // Deref coercion is a convenience that allows Rust to automatically convert
    // a type that implements `Deref` into a reference to the target type,
    // usually in function arguments or method calls.

    println!("\n--- 3. Implicit Deref Coercion in Functions ---");

    fn print_length(s: &str) {
        println!("Length of string: {}", s.len());
    }

    let owned_string = String::from("Rust is awesome!");
    // Pass value to function using ampersand (regular reference)
    print_length(&owned_string); // `&String` automatically derefs to `&str`

    let boxed_string = Box::new(String::from("Boxed string!"));
    // Deref coercion: `Box<String>` can be coerced to `&String`, then `&str`
    print_length(&boxed_string); // `&Box<String>` automatically derefs to `&String` then `&str`

    let my_boxed_string = MyBox::new(String::from("MyBoxed string!"));
    // Deref coercion also works for our custom `MyBox` because it implements `Deref`
    print_length(&my_boxed_string); // `&MyBox<String>` automatically derefs to `&String` then `&str`

    // -------------------------------------------------------------------------
    // 4. Rc<T>: Reference Counting (Shared Ownership)
    // -------------------------------------------------------------------------
    // `Rc<T>` (Reference Counted) is a smart pointer that enables multiple ownership
    // of the same data. It keeps a count of how many `Rc` pointers are pointing
    // to a value. The value is dropped only when the count reaches zero.
    // This is useful for graphs or when you need multiple parts of your program
    // to "own" the same piece of data.

    println!("\n--- 4. Rc<T>: Reference Counting ---");

    // Rc disallows mutation of the wrapped value (unless combined with RefCell)
    let rc_data = Rc::new(vec![1, 2, 3]); // Cannot directly modify data inside Rc
    let rc_clone_a = Rc::clone(&rc_data); // Clone creates a new Rc pointing to the same data, increments count
    let rc_clone_b = rc_data.clone(); // `clone()` method also calls `Rc::clone()`
    println!("Rc data: {:?}", rc_data);
    println!("Reference count: {}", Rc::strong_count(&rc_data));

    // Rc is single-threaded explanation:
    // `Rc` is *not* safe for use across multiple threads because its reference count
    // is not atomic. In a multi-threaded context, you would use `Arc<T>` (Atomic Reference Counted).

    // Create a vector of string objects
    let shared_vec = Rc::new(vec![
        String::from("apple"),
        String::from("banana"),
        String::from("cherry"),
    ]);
    println!("\nOriginal Rc vector: {:?}", shared_vec);
    println!(
        "Strong count after creation: {}",
        Rc::strong_count(&shared_vec)
    );

    let consumer1 = Rc::clone(&shared_vec);
    println!(
        "Strong count after consumer1 clone: {}",
        Rc::strong_count(&shared_vec)
    );

    let consumer2 = Rc::clone(&shared_vec);
    println!(
        "Strong count after consumer2 clone: {}",
        Rc::strong_count(&shared_vec)
    );

    // -------------------------------------------------------------------------
    // 5. Weak References to Rc<T>
    // -------------------------------------------------------------------------
    // `Weak<T>` is a non-owning smart pointer that complements `Rc<T>`.
    // It does not contribute to the strong reference count, meaning its existence
    // will not prevent the `Rc` from being dropped. This is crucial for breaking
    // reference cycles (e.g., in a doubly-linked list) which would otherwise
    // lead to memory leaks.

    println!("\n--- 5. Weak References to Rc<T> ---");

    // Get a weak reference to the Rc
    let weak_ref: Weak<Vec<String>> = Rc::downgrade(&shared_vec);
    println!(
        "Weak reference created. Strong count: {}",
        Rc::strong_count(&shared_vec)
    );
    println!("Weak count: {}", Rc::weak_count(&shared_vec));

    // Try to upgrade the weak reference while the Rc is still alive
    if let Some(upgraded_rc) = weak_ref.upgrade() {
        println!("Successfully upgraded weak reference: {:?}", upgraded_rc);
        println!(
            "Strong count after upgrade check: {}",
            Rc::strong_count(&shared_vec)
        );
    } else {
        println!("Failed to upgrade weak reference (should not happen yet).");
    }

    // Drop the original Rc
    drop(shared_vec);
    println!(
        "\nOriginal Rc dropped. Strong count: {}",
        Rc::strong_count(&rc_clone_a)
    ); // Strong count is now 2 (rc_clone_a, rc_clone_b)
    println!("Weak count: {}", Rc::weak_count(&rc_clone_a));

    drop(rc_clone_a);
    println!(
        "First clone dropped. Strong count: {}",
        Rc::strong_count(&rc_clone_b)
    ); // Strong count is now 1 (rc_clone_b)
    println!("Weak count: {}", Rc::weak_count(&rc_clone_b));

    drop(rc_clone_b);
    println!("Second clone dropped. Strong count: 0"); // Strong count is now 0
    println!("Weak count: {}", weak_ref.weak_count()); // Weak count is 0 if no more Weak references exist, but the Weak ref itself still exists.

    // Crash when upgrading and unwrapping weak reference (without match)
    // This demonstrates that weak references won't hold onto the underlying data.
    // If you try to `unwrap()` the result of `upgrade()` when the data is gone, it will panic.
    println!("\nAttempting to upgrade weak reference after all strong references are dropped:");
    // let doomed_rc = weak_ref.upgrade().unwrap(); // This line would panic!
    // println!("Doomed RC: {:?}", doomed_rc);

    // Use match on result of upgrade() to handle the potential `None`
    match weak_ref.upgrade() {
        Some(rc) => println!(
            "Successfully upgraded weak reference (after drop): {:?}",
            rc
        ),
        None => println!("Failed to upgrade weak reference: The data has been dropped."),
    }

    // -------------------------------------------------------------------------
    // 6. Mutability with Pointers: Cell and RefCell
    // -------------------------------------------------------------------------
    // `Rc` disallows mutation of the wrapped value directly. To achieve "interior mutability"
    // (modifying data through an immutable reference), Rust provides `Cell<T>` and `RefCell<T>`.
    // These types allow you to circumvent Rust's usual borrowing rules at runtime,
    // making them powerful but requiring careful use.

    println!("\n--- 6. Mutability with Pointers: Cell and RefCell ---");

    // 6.1 Cell<T>: Copying Values (for Copy types)
    // `Cell<T>` is used for types that implement `Copy` (like primitives, `char`, `bool`).
    // It allows you to get and set the inner value through an immutable reference to the `Cell`.

    println!("\n--- 6.1 Cell<T> ---");

    // Create a Person struct with Cell of age
    struct Person {
        name: String,
        age: Cell<u8>, // Allows modifying `age` even if `Person` is immutable
    }

    impl Person {
        fn new(name: &str, age: u8) -> Person {
            Person {
                name: name.to_string(),
                age: Cell::new(age),
            }
        }

        // Add a function to increment age
        fn increment_age(&self) {
            let current_age = self.age.get(); // Get a copy of the current age
            self.age.set(current_age + 1); // Set the new age
        }

        fn print_age(&self) {
            println!("{}'s age: {}", self.name, self.age.get());
        }
    }

    // Create a new instance of Person
    let alice = Person::new("Alice", 30); // `alice` is immutable
    alice.print_age();

    // Increment the age and print it
    alice.increment_age();
    alice.print_age();

    // Cell allows interior mutability:
    // Even though `alice` is declared `let alice`, we can still modify its `age` field
    // because `age` is wrapped in a `Cell`. This is safe because `Cell` works by
    // copying the value, which doesn't invalidate references.

    // 6.2 RefCell<T>: Borrowing Values (for any type)
    // `RefCell<T>` allows interior mutability for *any* type, including non-`Copy` types.
    // Unlike `Cell`, `RefCell` enforces borrowing rules at *runtime*.
    // It keeps track of how many immutable or mutable borrows are active.

    // RefCell is only allowed in single-threaded environments:
    // Like `Rc`, `RefCell` is not thread-safe. For multi-threaded interior mutability,
    // you would use `Mutex<T>` or `RwLock<T>`.

    println!("\n--- 6.2 RefCell<T> ---");

    let my_vec: RefCell<Vec<i32>> = RefCell::new(vec![10, 20, 30]);

    // RefCell can be borrowed immutably or mutably:
    // Get an immutable reference to the vector
    let borrowed_immut = my_vec.borrow();
    println!("Immutably borrowed vector: {:?}", *borrowed_immut);

    // Get a mutable reference to the vector
    let mut borrowed_mut = my_vec.borrow_mut();
    borrowed_mut.push(40);
    println!("Mutably borrowed and modified vector: {:?}", *borrowed_mut);
    drop(borrowed_mut); // Important: Drop the mutable borrow to allow other borrows

    // And example of where RefCell panics at runtime
    println!("\n--- RefCell Runtime Panic Example ---");

    let data_vec = RefCell::new(vec![1, 2, 3]);

    // Get an immutable reference to the vector
    let r1 = data_vec.borrow();
    println!("First immutable borrow: {:?}", *r1);

    // Attempt to get a mutable reference while an immutable one is active (runtime panic!)
    // This adheres to Rust's borrowing rules: you cannot have a mutable borrow
    // when an immutable one is active. RefCell enforces this at runtime.
    /*
    let r2 = data_vec.borrow_mut(); // This line will panic!
    println!("Second mutable borrow: {:?}", *r2);
    */
    println!(
        "Attempting to get a mutable borrow while an immutable one is active would cause a runtime panic. (Example commented out)"
    );

    // Get a mutable reference to the vector
    let mut_vec_ref = data_vec.borrow_mut();
    println!("Mutably borrowed: {:?}", *mut_vec_ref);

    // Push a new value to the vector
    mut_vec_ref.push(100);
    println!("Value pushed. Mutably borrowed: {:?}", *mut_vec_ref);

    // Print the length (after dropping the mutable borrow)
    drop(mut_vec_ref); // Release the mutable borrow
    println!("Length after modification: {}", data_vec.borrow().len());

    // -------------------------------------------------------------------------
    // 7. Combining Pointers!
    // -------------------------------------------------------------------------
    // Rust's smart pointers can be combined to achieve complex ownership and
    // mutability patterns. For example, `Rc<RefCell<T>>` is a common pattern
    // for shared, mutable data in a single-threaded context.

    println!("\n--- 7. Combining Pointers! ---");
    let shared_mutable_data = Rc::new(RefCell::new(vec!['a', 'b']));

    let r_clone1 = Rc::clone(&shared_mutable_data);
    let r_clone2 = Rc::clone(&shared_mutable_data);

    // Modify data through one of the Rc clones
    {
        let mut borrowed_data = r_clone1.borrow_mut();
        borrowed_data.push('c');
    }
    println!(
        "Shared mutable data after modification: {:?}",
        *r_clone2.borrow()
    );
    println!(
        "Strong count for combined pointer: {}",
        Rc::strong_count(&r_clone1)
    );

    // -------------------------------------------------------------------------
    // 8. Learning Pointers in Rust is a Must!
    // -------------------------------------------------------------------------
    // While Rust aims to minimize the direct use of raw pointers, understanding
    // how references, smart pointers, and their underlying memory models work
    // is absolutely fundamental to writing idiomatic, safe, and performant Rust code.
    // It empowers you to:
    // - Reason about memory safety and ownership.
    // - Choose the right smart pointer for your data structures.
    // - Write efficient code by understanding stack vs. heap allocation.
    // - Interact with external C libraries safely.
    // - Debug ownership and borrowing issues effectively.

    println!("\n--- End of Advanced Pointers Examples ---");
}
