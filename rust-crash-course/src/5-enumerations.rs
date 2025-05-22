fn main() {
    // -------------------------------------------------------------------------
    // Example 1: Defining a Simple Enum and Creating Instances
    // -------------------------------------------------------------------------
    // An `enum` allows you to define a type by listing all its possible variants.
    // Think of it as a custom data type where you explicitly name all the valid
    // forms it can take.
    // Useful for related objects and data

    enum TrafficLight {
        Red,
        Yellow,
        Green,
    }

    let current_light: TrafficLight = TrafficLight::Red;
    println!("Current traffic light is Red.");
    // `current_light` is a variable of type `TrafficLight`, holding the `Red` variant.
    // It can only be one of the variants defined in `TrafficLight`.

    let next_light: TrafficLight = TrafficLight::Green;
    println!("Next traffic light will be Green.");
}

fn main() {
    // -------------------------------------------------------------------------
    // Example 2: Enums with Associated Data (Tuples and Structs)
    // -------------------------------------------------------------------------
    // Enum variants can carry data, allowing you to attach values to each variant.
    // This makes enums incredibly flexible for representing diverse information.

    // Define a struct to be used inside an enum variant
    struct User {
        id: u32,
        name: String,
    }

    enum Message {
        Quit,                       // No data
        Move { x: i32, y: i32 },    // Anonymous struct (named fields)
        Write(String),              // Tuple with a single String
        ChangeColor(i32, i32, i32), // Tuple with three i32s (RGB)
        Enroll(User),               // Holds an instance of the `User` struct
    }

    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Write(String::from("Hello, Rust!"));
    let msg4 = Message::ChangeColor(255, 128, 0);
    let user_alice = User {
        id: 1,
        name: String::from("Alice"),
    };
    let msg5 = Message::Enroll(user_alice);

    println!("\nExample of enums with associated data:");
    // We can't directly print enums without deriving `Debug`,
    // but we can see their creation.
}

fn main() {
    // -------------------------------------------------------------------------
    // Example 3: Using `match` Expressions (Exhaustive Pattern Matching)
    // -------------------------------------------------------------------------
    // The `match` control flow operator compares a value against a series of patterns
    // and executes code based on the first pattern that matches. It's exhaustive,
    // meaning you must cover every possible case.

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState), // Variant holding another enum
    }

    #[derive(Debug)] // Added for printing purposes
    enum UsState {
        Alabama,
        Alaska,
        // ... and so on
        California,
        Texas,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            // Destructuring the Quarter variant to get the associated state
            Coin::Quarter(state) => {
                println!("Quarter from {:?}", state);
                25
            }
        }
    }

    println!("\nExample of `match` with Coin enum:");
    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let quarter = Coin::Quarter(UsState::California);

    println!("A penny is worth {} cents.", value_in_cents(penny));
    println!("A nickel is worth {} cents.", value_in_cents(nickel));
    println!("A quarter is worth {} cents.", value_in_cents(quarter));
}

fn main() {
    // -------------------------------------------------------------------------
    // Example 4: `match` with Numeric Values and `_` (Catch-all)
    // -------------------------------------------------------------------------
    // `match` is not just for enums; it can be used with any type.
    // The `_` (underscore) pattern acts as a wildcard, matching any value
    // not covered by previous patterns, ensuring exhaustiveness.

    let number = 15;

    println!("\nExample of `match` with numeric values:");
    match number {
        1 => println!("The number is one."),
        2 | 3 => println!("The number is two or three."), // Multiple patterns with `|` (OR)
        4..=10 => println!("The number is between four and ten (inclusive)."), // Range pattern
        // `val @ 11..=20` is an "at" pattern. It matches the range,
        // and also binds the matched value to a new variable `val`.
        val @ 11..=20 => println!("The number is {} and it's between eleven and twenty.", val),
        _ => println!("The number is something else."), // Catch-all
    }
}

fn main() {
    // -------------------------------------------------------------------------
    // Example 5: `if let` (Concise Single-Pattern Matching)
    // -------------------------------------------------------------------------
    // `if let` is a concise way to handle a single pattern match, where you
    // only care about one specific variant or pattern and want to ignore others.
    // It's often used as a shorthand for `match` when you only need to execute
    // code if the value matches one pattern.

    enum OptionalValue {
        Present(String),
        Absent,
    }

    let config_setting = OptionalValue::Present(String::from("debug"));
    let user_preference = OptionalValue::Absent;

    println!("\nExample of `if let`:");

    if let OptionalValue::Present(value) = config_setting {
        println!("Configuration setting found: {}", value);
    } else {
        println!("Configuration setting is absent.");
    }

    if let OptionalValue::Present(value) = user_preference {
        println!("User preference found: {}", value);
    } else {
        println!("User preference is absent.");
    }

    // `if let` is commonly used with `Option` and `Result` enums,
    // which are fundamental for error handling and optional values in Rust.
    // (Rust's built-in `Option` and `Result` are widely used).
}

fn main() {
    // -------------------------------------------------------------------------
    // Example 6: Enums with Methods
    // -------------------------------------------------------------------------
    // Just like structs, you can define methods on enums using an `impl` block.
    // The method's behavior can differ based on the enum variant, allowing for
    // polymorphic behavior.

    enum Shape {
        Circle { radius: f64 },
        Rectangle { width: f64, height: f64 },
        Triangle(f64, f64, f64), // Sides
    }

    impl Shape {
        // Method to calculate the area of the shape
        fn area(&self) -> f64 {
            match self {
                Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
                Shape::Rectangle { width, height } => width * height,
                Shape::Triangle(s1, s2, s3) => {
                    // Heron's formula for triangle area
                    let s = (s1 + s2 + s3) / 2.0;
                    (s * (s - s1) * (s - s2) * (s - s3)).sqrt()
                }
            }
        }

        // Method to describe the shape
        fn describe(&self) {
            match self {
                Shape::Circle { radius } => {
                    println!("This is a Circle with radius {}.", radius);
                }
                Shape::Rectangle { width, height } => {
                    println!(
                        "This is a Rectangle with width {} and height {}.",
                        width, height
                    );
                }
                Shape::Triangle(s1, s2, s3) => {
                    println!("This is a Triangle with sides {}, {}, {}.", s1, s2, s3);
                }
            }
        }
    }

    println!("\nExample of Enums with Methods:");
    let my_circle = Shape::Circle { radius: 7.0 };
    let my_rectangle = Shape::Rectangle {
        width: 8.0,
        height: 5.0,
    };
    let my_triangle = Shape::Triangle(3.0, 4.0, 5.0); // A right triangle

    my_circle.describe();
    println!("Area: {:.2}", my_circle.area());

    my_rectangle.describe();
    println!("Area: {:.2}", my_rectangle.area());

    my_triangle.describe();
    println!("Area: {:.2}", my_triangle.area());
}
