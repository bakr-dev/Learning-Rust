// This file covers the fundamental concepts of variables, ownership, and borrowing in Rust.

fn main() {
    // -------------------------------------------------------------------------
    // 1. Variables and Ownership
    // -------------------------------------------------------------------------
    // In Rust, every value has a variable that's called its *owner*.
    // There can only be one owner of a value at a time.
    // When the owner goes out of scope, the value will be dropped (deallocated).
    // Ownership is how Rust manages memory.

    let s1 = String::from("hello"); // s1 owns the String data on the heap
    println!("s1: {}", s1);

    // -------------------------------------------------------------------------
    // 2. Moving Ownership
    // -------------------------------------------------------------------------
    // When you assign a non-copy type (like String) to another variable,
    // the ownership of the value moves. The first variable is then invalid.
    // This is to prevent double free error.

    let s2 = s1; // Ownership of the String data moves from s1 to s2
    println!("s2: {}", s2);
    // println!("s1: {}", s1); // This would result in a compile-time error: "value borrowed here after move"
    // s1 is no longer valid after the move to s2

    // -------------------------------------------------------------------------
    // 3. Borrowing: References
    // -------------------------------------------------------------------------
    // Borrowing allows you to use a value without taking ownership.
    // References are like pointers but Rust guarantees they always point to a valid value.
    // References do not own the data they point to.

    let s3 = String::from("world");
    let r1 = &s3; // r1 is an immutable reference to s3
    let r2 = &s3; // You can have multiple immutable references to the same value

    println!("r1: {}, r2: {}, s3: {}", r1, r2, s3);
    // s3 is still valid, because r1 and r2 only borrowed its data.

    // -------------------------------------------------------------------------
    // 4. Mutable References
    // -------------------------------------------------------------------------
    // You can also create mutable references, which allow you to change the value.
    // However, there's a strict rule: you can have either one mutable reference
    // to a value or any number of immutable references, but not both at the same time
    // within the same scope. This is one of the key rules that enables Rust to prevent data races at compile time.

    let mut s4 = String::from("hello");
    let r3 = &mut s4; // r3 is a mutable reference to s4
    r3.push_str(", rust!"); // change the value that r3 refers to.
    println!("s4: {}", s4); // s4 has been changed.

    // let r4 = &s4; // This would cause a compile-time error: cannot borrow `s4` as immutable because it is also borrowed as mutable
    println!("r3: {}", r3);

    // -------------------------------------------------------------------------
    // 5. Scope and Variable Validity
    // -------------------------------------------------------------------------
    // Variables are only valid within the scope they are declared.
    // When a variable goes out of scope, Rust automatically calls the `drop` function
    // for its value, freeing the associated memory (for types like String).
    // Scope is the range within a program where an item is valid.

    {
        let s5 = String::from("in scope");
        println!("s5: {}", s5);
    } // s5 goes out of scope here, and the memory for the String is dropped

    // println!("s5: {}", s5); // This would be an error: `s5` is not in scope

    // -------------------------------------------------------------------------
    // 6. Accessing Block Variables Outside (Not Directly Possible)
    // -------------------------------------------------------------------------
    // Variables declared inside a block (delimited by `{}`) are only accessible within that block.
    // To use a value from inside a block outside, you need to either:
    //    - Return it from a function.
    //    - Declare it outside the block and modify it inside (if mutable).
    //    - Move the ownership of the variable.

    let mut s6 = String::new();
    {
        let s_inner = String::from("inner value");
        s6 = s_inner; // Ownership moves out of the block
        // s_inner goes out of scope here.  The memory owned by s_inner is now owned by s6
    }
    println!("s6: {}", s6);

    // -------------------------------------------------------------------------
    // 7. Rust's Memory Management and "Garbage Collection" (No Traditional GC)
    // -------------------------------------------------------------------------
    // Rust does not have a traditional garbage collector like Java or Python.
    // Instead, it uses a system of ownership with a set of rules that the compiler
    // enforces at compile time. This ensures memory safety without the overhead
    // of a runtime garbage collector. Memory is automatically deallocated when
    // the owner of the data goes out of scope. This is often referred to as
    // "RAII" (Resource Acquisition Is Initialization).  Rust's memory management is deterministic.

    // -------------------------------------------------------------------------
    // 8. Integer vs. String: Stack vs. Heap
    // -------------------------------------------------------------------------
    // Integers (fixed-size primitives) are typically stored on the stack.
    // The stack is fast and memory allocation/deallocation is very efficient.
    // Data on the stack must have a known, fixed size.
    let x = 5; // x is stored on the stack
    let y = x; // Copy: the value of x is copied to y (stack-based)
    println!("x: {}, y: {}", x, y);

    // Strings (growable, non-fixed size) are stored on the heap.
    // The String variable itself on the stack holds a pointer to the heap-allocated data,
    // the length, and the capacity.
    let s7 = String::from("hello"); // The string content is on the heap
    let s8 = s7.clone(); // Clone: creates a deep copy of the heap data, including the heap allocation.
    println!("s7: {}, s8: {}", s7, s8);

    // Without .clone(), `let s8 = s7;` would move ownership, invalidating `s7`.

    // -------------------------------------------------------------------------
    // 9. Scenario: Two String Variables Pointing to the Same Heap (Prevented by Ownership)
    // -------------------------------------------------------------------------
    // Rust's ownership rules prevent two `String` variables from directly pointing
    // to the same heap data and both trying to free it when they go out of scope.
    // This is the "double free" error that Rust's memory management system avoids.
    // Rust prevents data races.

    let s9 = String::from("shared data");
    // let s10 = &s9; // This is borrowing, not ownership transfer
    // // When s9 goes out of scope, the data is freed. s10 is still a valid reference.

    let s11 = s9; // Ownership moves to s11. s9 is no longer valid.
    // // If s11 and s9 both tried to free the same memory, it would lead to an error
    // // in other languages. Rust prevents this at compile time.

    println!("s11: {}", s11);

    // -------------------------------------------------------------------------
    // 10. Moving Approach on the Previous Example (Ownership Transfer)
    // -------------------------------------------------------------------------
    // As demonstrated above, the "moving" approach involves transferring ownership
    // from one variable to another. After the move, the original variable is no
    // longer valid. This ensures that only one owner is responsible for freeing
    // the memory.

    let s12 = String::from("move example");
    let s13 = s12; // Ownership moved from s12 to s13
    println!("s13: {}", s13);
    // println!("s12: {}", s12); // Error: s12 is no longer valid

    // -------------------------------------------------------------------------
    // 11. Copy and Move on the Stack (Integers) and Heap (String)
    // -------------------------------------------------------------------------
    // Types that have a known size at compile time and can be trivially copied
    // (like integers, booleans, characters, tuples of copyable types) implement
    // the `Copy` trait. When you assign a `Copy` type, its value is copied.
    // Copy trait:  The old variable remains valid after assignment.

    let i1 = 10; // Stored on the stack
    let i2 = i1; // Value of i1 is copied to i2 (both are valid)
    println!("i1: {}, i2: {}", i1, i2);

    // Non-`Copy` types like `String` are moved by default. To create a separate
    // copy on the heap, you need to use the `.clone()` method.
    // Move:  The old variable is invalid after assignment.

    let str1 = String::from("heap data"); // Data on the heap
    let str2 = str1.clone(); // Deep copy of the heap data
    println!("str1: {}, str2: {}", str1, str2);

    // -------------------------------------------------------------------------
    // 12. References: Borrowing Without Taking Ownership
    // -------------------------------------------------------------------------
    // References allow you to access and use a value without transferring ownership.
    // They are created using the `&` operator.

    let s14 = String::from("borrowing");
    let r_s14 = &s14; // Immutable reference to s14

    println!("s14: {}", s14);
    println!("r_s14: {}", r_s14);

    // -------------------------------------------------------------------------
    // 13. References and Function Parameters
    // -------------------------------------------------------------------------
    // When you pass a variable to a function, the ownership might be moved.
    // To prevent this and allow the function to use the value without taking
    // ownership, you can pass references as parameters.

    fn print_string(s: &String) {
        // s is a reference to a String
        println!("Function received: {}", s);
    } // s goes out of scope, but it doesn't own the String, so no drop occurs

    let my_string = String::from("hello from function");
    print_string(&my_string); // Pass a reference to my_string
    println!("my_string after function call: {}", my_string); // my_string is still valid

    fn take_ownership(s: String) {
        // s takes ownership of the String
        println!("Function got ownership: {}", s);
    } // s goes out of scope, and the String is dropped

    let another_string = String::from("taking ownership");
    take_ownership(another_string); // Ownership of another_string moves to the function
    // println!("another_string after function call: {}", another_string); // Error: another_string is no longer valid

    // -------------------------------------------------------------------------
    // 14. Mutable and Immutable References: Rules and Use Cases
    // -------------------------------------------------------------------------
    // - Immutable References (`&T`): Allow you to read the data but not modify it.
    //   You can have multiple immutable references to the same value at the same time.
    //   Use cases: Reading data without needing to change it, allowing multiple parts
    //   of your code to access the same data concurrently without risk of modification.

    let data = vec![1, 2, 3];
    let first = &data[0];
    let second = &data[1];
    println!("First: {}, Second: {}", first, second);

    // - Mutable References (`&mut T`): Allow you to modify the data.
    //   You can have at most one mutable reference to a particular piece of data
    //   in a specific scope.
    //   Use cases: Modifying data in place, ensuring exclusive access to a resource
    //   to prevent data corruption.

    let mut counter = 0;
    let increment = &mut counter;
    *increment += 1;
    println!("Counter: {}", counter);

    // -------------------------------------------------------------------------
    // 15. Creating Mutable Variables for Functions
    // -------------------------------------------------------------------------
    // To allow a function to modify a variable through a mutable reference,
    // the variable itself must be declared as mutable using the `mut` keyword.

    fn modify_string(s: &mut String) {
        s.push_str(" (modified)");
    }

    let mut changeable_string = String::from("initial");
    modify_string(&mut changeable_string);
    println!("changeable_string: {}", changeable_string);

    // let immutable_string = String::from("not changeable");
    // modify_string(&mut immutable_string); // Error: cannot borrow `immutable_string` as mutable, as it is not declared as mutable

    // -------------------------------------------------------------------------
    // 16. At Most One Mutable Reference at a Time
    // -------------------------------------------------------------------------
    // Rust enforces the rule that you can have at most one mutable reference
    // to a piece of data in a particular scope. This prevents data races.
    // Data Race:  Two or more pointers access the same memory location at the same time,
    // at least one of them is writing, and there is no mechanism being used to synchronize access to that data.

    let mut value = 10;
    let ref1 = &mut value;
    // let ref2 = &mut value; // Compile-time error: cannot borrow `value` as mutable more than once at a time

    *ref1 += 5;
    println!("Value: {}", value);

    // The scope of `ref1` ends here.

    let ref3 = &mut value; // Now you can create another mutable reference
    *ref3 *= 2;
    println!("Value after ref3: {}", value);

    // -------------------------------------------------------------------------
    // 17. Mutable References Prevent Data Races
    // -------------------------------------------------------------------------
    // A data race occurs when two or more pointers access the same memory location
    // at the same time, at least one of them is writing, and there is no mechanism
    // being used to synchronize access to that data.

    // Rust's rule of allowing only one mutable reference (or multiple immutable
    // references) at a time at compile time prevents these conditions from occurring.
    // If you have a mutable reference, you have exclusive access to the data,
    // so no other part of the code can modify it simultaneously.

    // Imagine a scenario in a multi-threaded environment (though this example is single-threaded):
    // If two threads could simultaneously have mutable access to the same data,
    // the outcome would be unpredictable. Rust's borrowing rules prevent this.

    // -------------------------------------------------------------------------
    // 18. No Mutable References While Immutable References Exist
    // -------------------------------------------------------------------------
    // You cannot have a mutable reference if there are any immutable references
    // active in the same scope. This prevents the data being mutated unexpectedly
    // while other parts of the code are reading it, ensuring data consistency.
    {
        let data2 = vec![10, 20, 30];
        let immutable_ref1 = &data2[0];
        let immutable_ref2 = &data2[1];
        println!("Immutable refs: {}, {}", immutable_ref1, immutable_ref2);

        let mutable_ref = &mut data2; // Compile-time error: cannot borrow `data2` as mutable because it is also borrowed as immutable
    }

    // When The immutable references go out of scope.

    let mutable_ref2 = &mut data2; // Now you can have a mutable reference
    mutable_ref2.push(40);
    println!("Modified data2: {:?}", data2);

    // -------------------------------------------------------------------------
    // 19. Trying Mutable References When Immutable References Are Out of Scope
    // -------------------------------------------------------------------------
    // As seen in the previous example, you can obtain a mutable reference once
    // any existing immutable references to the same data are no longer in scope.
    // The scope is determined by the curly braces `{}`.

    let data3 = vec![5, 6, 7];
    {
        let immutable_r = &data3[0];
        println!("Inside scope: {}", immutable_r);
    } // immutable_r goes out of scope here

    let mutable_r = &mut data3; // Now it's allowed
    mutable_r.push(8);
    println!("Data3 after mutable borrow: {:?}", data3);

    // -------------------------------------------------------------------------
    // 20. Dangling References and Returning References from Functions
    // -------------------------------------------------------------------------
    // A dangling reference is a reference that points to memory that has been
    // deallocated. Rust's borrow checker prevents dangling references at compile time.
    // Dangling Pointer: A pointer that points to invalid memory

    // Example of a function that would create a dangling reference (and Rust will prevent it):
    // fn dangle() -> &String { // Returns a reference to a String
    //     let s = String::from("hello"); // s is created inside the function
    //     &s // Returns a reference to s. s will be dropped when the function ends.
    // }
    //
    // // let dangling_ref = dangle(); // dangling_ref would be pointing to invalid memory

    // To return a string from a function, you should return the `String` itself
    // (ownership is transferred) or a reference with a lifetime that is tied
    // to something outside the function.

    fn no_dangle() -> String {
        // Returns an owned String
        let s = String::from("hello");
        s // Ownership of s is moved out of the function
    }

    let safe_string = no_dangle();
    println!("Safe string: {}", safe_string);

    // -------------------------------------------------------------------------
    // 21. Different Ownership Scenarios (Summary)
    // -------------------------------------------------------------------------
    // 1. Moving: When a non-`Copy` type is assigned to another variable, ownership moves.
    //    The original variable is no longer valid.
    // 2. Borrowing (References): Creating references (`&` for immutable, `&mut` for mutable)
    //    allows you to use a value without taking ownership.
    // 3. Copying: Types that implement the `Copy` trait are copied when assigned. Both
    //    variables are valid and own their own data.  Stack-only data can be Copy.
}
