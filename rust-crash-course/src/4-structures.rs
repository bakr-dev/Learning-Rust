// This file covers the fundamental concepts of structures in Rust.

fn main() {
    // -------------------------------------------------------------------------
    // 1. Defining Structures
    // -------------------------------------------------------------------------
    // Structures group related data, creating custom types.
    // Use `struct` keyword, and define fields with names and types.

    // Structure representing a point in 2D space.
    struct Point {
        x: i32, // X-coordinate as a 32-bit integer
        y: i32, // Y-coordinate as a 32-bit integer
    }

    let p1 = Point { x: 5, y: 10 }; // Instance of Point
    println!("Point p1: x = {}, y = {}", p1.x, p1.y);

    // Structure with varied data types.
    struct User {
        username: String,   // String for username
        email: String,      // String for email
        sign_in_count: u64, // Unsigned 64-bit integer
        active: bool,       // Boolean for activity status
    }

    let user1 = User {
        username: String::from("Ahmed"),
        email: String::from("ahmed@example.com"),
        sign_in_count: 1,
        active: true,
    };

    println!("User username: {}, email: {}", user1.username, user1.email);

    // -------------------------------------------------------------------------
    // 2. Types of Structures
    // -------------------------------------------------------------------------
    // Rust offers three structure types: named, tuple, and unit.

    // a. Named Structures: Fields have names (e.g., Point, User).
    //    Clear and descriptive.

    // b. Tuple Structures: Fields lack names, accessed by index.
    //    Useful for simple data encapsulation.
    struct Color(i32, i32, i32); // RGB Color

    let black = Color(0, 0, 0);
    println!("Black color: R={}, G={}, B={}", black.0, black.1, black.2);

    // Example with more descriptive tuple struct
    struct Dimensions(u32, u32, u32); // 3D Dimensions (width, height, depth)
    let cube_dimensions = Dimensions(10, 20, 30);
    println!(
        "Cube dimensions: width={}, height={}, depth={}",
        cube_dimensions.0, cube_dimensions.1, cube_dimensions.2
    );

    // c. Unit Structures: No fields, used as a marker or placeholder.
    struct FileDescriptor; // Represents an open file.

    let file1 = FileDescriptor;
    println!("File Descriptor created");

    // -------------------------------------------------------------------------
    // 3. Using Structures
    // -------------------------------------------------------------------------
    // Access fields with the dot operator (`.`).
    // Create instances and assign field values.
    // Structures are immutable by default.  To modify a structure, the instance must be declared as mutable using `mut`.

    let mut user2 = User {
        // user2 is mutable
        username: String::from("Mohammed"),
        email: String::from("mohammed@example.com"),
        sign_in_count: 2,
        active: true,
    };

    user2.email = String::from("mohammed_updated@example.com"); // Modify email.  This is allowed because user2 is mutable.
    println!("User new email: {}", user2.email);

    // let user3 = User{
    //     username: String::from("invalid"),
    //     email: String::from("invalid@email.com"),
    //     sign_in_count: 3,
    //     active: false,
    // };
    // user3.email = String::from("new_email@email.com"); //This is not allowed. user3 is not declared as mutable.

    // -------------------------------------------------------------------------
    // 4. Associated Functions
    // -------------------------------------------------------------------------
    // Functions associated with a struct, not methods.  No `self` parameter.
    // Often used as constructors (like `new`).

    impl User {
        fn new(email: String, username: String) -> User {
            User {
                email,
                username,
                active: true,
                sign_in_count: 0,
            }
        }
    }

    let user3 = User::new(String::from("Sara"), String::from("sara@example.com"));
    println!("User3 username: {}, email: {}", user3.username, user3.email);

    // -------------------------------------------------------------------------
    // 5. Methods
    // -------------------------------------------------------------------------
    // Functions within a struct's context. Take `self`, `&self`, or `&mut self`.
    //    - `self` :  Takes ownership of the instance.
    //    - `&self`:  Borrows the instance immutably.
    //    - `&mut self`: Borrows the instance mutably.

    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        // Calculates the area of the rectangle.
        fn area(&self) -> u32 {
            //Immutable borrow of self
            self.width * self.height
        }

        // Checks if this rectangle can contain another rectangle.
        fn can_hold(&self, other: &Rectangle) -> bool {
            //Immutable borrow of other
            self.width > other.width && self.height > other.height
        }

        //Creates a square rectangle
        fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }

        fn set_width(&mut self, new_width: u32) {
            //Mutable borrow of self
            self.width = new_width;
        }
    }

    struct Color(i32, i32, i32); // RGB Color

    impl Color {
        fn get_red(&self) -> i32 {
            self.0
        }
        fn set_red(&mut self, new_red: i32) {
            self.0 = new_red;
        }

        fn create_color(r: i32, g: i32, b: i32) -> Self {
            Color(r, g, b)
        }
    }

    let mut rect1 = Rectangle {
        //rect1 is mutable.  Note that rect1 must be declared as mutable to allow calling the mutable set_width method.
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let mut c1 = Color(200, 0, 0);

    println!("Area of rect1: {}", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    let square1 = Rectangle::square(20);
    println!("Area of square1: {}", square1.area());

    rect1.set_width(35); //Change width using a mutable method.
    println!("New width of rect1: {}", rect1.width);
    println!("c1 red value: {}", c1.get_red());
    c1.set_red(255);
    println!("c1 new red value: {}", c1.0); // Accessing tuple struct field directly
    // Using the method for consistency, though direct access is fine here.
    println!("c1 new red value (via method): {}", c1.get_red());

    let c2 = Color::create_color(100, 150, 200);
    println!("c2 color values R={}, G={}, B={}", c2.0, c2.1, c2.2);

    // -------------------------------------------------------------------------
    // 6. Instance Inheritance (Not traditional inheritance)
    // -------------------------------------------------------------------------
    //  Use struct update syntax for creating a new instance from another.

    let user4 = User {
        email: String::from("amal@example.com"),
        username: String::from("Amal"),
        ..user3 // Copy remaining fields from user3
    };

    println!("User4 username: {}, email: {}", user4.username, user4.email);

    // -------------------------------------------------------------------------
    // 7. Multiple Implementations
    // -------------------------------------------------------------------------
    // Rust allows you to have multiple `impl` blocks for a single struct.
    // This can be useful for organizing your code, such as grouping methods
    // by functionality or implementing different traits.

    impl Point {
        // Methods related to basic operations.
        fn get_x(&self) -> i32 {
            self.x
        }

        fn get_y(&self) -> i32 {
            self.y
        }
    }

    impl Point {
        // Methods related to geometric transformations.
        fn translate(&mut self, dx: i32, dy: i32) {
            self.x += dx;
            self.y += dy;
        }
    }

    let mut my_point = Point { x: 1, y: 2 };
    println!(
        "Original point: x = {}, y = {}",
        my_point.get_x(),
        my_point.get_y()
    );

    my_point.translate(3, 4);
    println!(
        "Translated point: x = {}, y = {}",
        my_point.get_x(),
        my_point.get_y()
    );

    // -------------------------------------------------------------------------
    // 8. The '.' (Dot) and '::' (Double Colon) Operators with Structs
    // -------------------------------------------------------------------------
    // These two operators are fundamental for interacting with structs in Rust,
    // but they serve distinctly different purposes.

    // 8.1. The '.' (Dot) Operator: Accessing Members of a Struct *Instance*
    // ---------------------------------------------------------------------
    // The dot operator is used to access fields (data) and instance methods
    // (functions defined in an `impl` block that take `self`, `&self`, or `&mut self`)
    // that belong to a *specific instance* of a struct.
    //
    // Think of it as navigating within a concrete object you've already created.
    // You need an *existing variable* that holds a struct instance to use the `.` operator.

    println!("\n--- Using the '.' (Dot) Operator ---");
    let mut my_user = User::new(
        String::from("dev_rust"),
        String::from("rust_dev@example.com"),
    );

    // Accessing a field of the 'my_user' instance
    println!("My user's username (via '.'): {}", my_user.username);

    // Calling an instance method on the 'my_user' instance
    // The `sign_in_count` here is a field on the instance.
    // The `email` field is also accessed directly on the instance.
    // This implicitly assumes a method like `get_info` if one were defined,
    // but here we're directly accessing fields via the dot operator.
    println!(
        "My user's sign in count (via '.'): {}",
        my_user.sign_in_count
    );

    // Modify a field using the dot operator (requires `mut` on the instance)
    my_user.sign_in_count += 1;
    println!(
        "My user's updated sign in count (via '.'): {}",
        my_user.sign_in_count
    );

    let mut small_rect = Rectangle {
        width: 5,
        height: 10,
    };
    // Calling an instance method `area` on the `small_rect` instance
    println!("Area of small_rect (via '.'): {}", small_rect.area());
    // Calling a mutable instance method `set_width` on the `small_rect` instance
    small_rect.set_width(7);
    println!("New width of small_rect (via '.'): {}", small_rect.width);

    // 8.2. The '::' (Double Colon) Operator: Associated Functions and Modules
    // -----------------------------------------------------------------------
    // The double colon (`::`) is used for "path-like" access.
    // Its primary uses with structs are:
    //
    // a. Calling **Associated Functions (Static Methods)**: These are functions
    //    that belong to the *struct type itself*, not a specific instance.
    //    They are often used for constructors (like `new`), factory methods,
    //    or utility functions that operate on data related to the struct's type
    //    but don't require an existing instance. You call them directly on the
    //    struct's name.
    //
    // b. Accessing Items within Modules: While not directly related to struct
    //    instances, `::` is also the standard way to navigate Rust's module
    //    system (e.g., `std::collections::HashMap`, `crate::my_module::MyStruct`).
    //    It's about referring to a type or item within its declared path.

    println!("\n--- Using the '::' (Double Colon) Operator ---");

    // Calling the `new` associated function on the `User` *type*
    // This creates a *new* User instance without needing an existing one.
    let created_user = User::new(String::from("john@example.com"), String::from("john_doe"));
    println!(
        "Created user username (via '::new'): {}",
        created_user.username
    );

    // Calling the `square` associated function on the `Rectangle` *type*
    let perfect_square = Rectangle::square(25);
    println!(
        "Perfect square area (via '::square'): {}",
        perfect_square.area()
    );

    // Calling the `create_color` associated function on the `Color` *type*
    let vibrant_color = Color::create_color(255, 100, 0);
    println!(
        "Vibrant color RGB (via '::create_color'): {}, {}, {}",
        vibrant_color.0, vibrant_color.1, vibrant_color.2
    );

    // General module path example (relevant to structs within modules)
    // Here, `Vec::new()` is an associated function of the `Vec` type in the standard library.
    let mut numbers = Vec::new(); // `Vec` is a struct (a generic one)
    numbers.push(10); // `push` is an instance method on the `numbers` Vec instance
    println!("Numbers vector: {:?}", numbers);
}
