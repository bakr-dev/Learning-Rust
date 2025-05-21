fn main() {
    // -------------------------------------------------------------------------
    // 1. Defining an Enumeration (Enum)
    // -------------------------------------------------------------------------
    // An `enum` allows you to define a type by listing all its possible variants.
    // Think of it as a custom data type where you explicitly name all the valid
    // forms it can take.

    // Define a simple struct that can be used inside an enum variant.
    // This struct now includes a tuple for contact information, showcasing nesting.
    struct PetOwner {
        name: String,
        age: u32,
        contact_info: (String, String), // Tuple for (phone_number, email)
    }

    enum AnimalType {
        Dog,               // A simple variant with no associated data.
        Cat,               // Another simple variant.
        Rabbit,            // Yet another simple variant.
        Bird(String, u32), // A variant that holds a tuple: (species: String, age: u32).
        // This is useful when the data has a clear order and meaning.
        // New variant: holds an instance of the `PetOwner` struct.
        // This demonstrates how enum variants can encapsulate more complex, named data structures.
        OwnedBy(PetOwner),
    }

    // -------------------------------------------------------------------------
    // 2. Creating an Enum Instance
    // -------------------------------------------------------------------------
    // To use an enum, you create an instance of one of its variants.
    // You access variants using the enum's name followed by `::` and the variant name.

    let fluffy: AnimalType = AnimalType::Dog;
    // `fluffy` is a variable of type `AnimalType`, holding the `Dog` variant.
    // It can only be one of the variants defined in `AnimalType`.

    let chirpy: AnimalType = AnimalType::Bird(String::from("Parrot"), 5);
    // Creating a `Bird` variant. The associated tuple data `("Parrot", 5)`
    // is directly provided when creating the instance.

    let owner_details = PetOwner {
        name: String::from("Alice"),
        age: 30,
        contact_info: (String::from("555-1234"), String::from("alice@example.com")),
    };
    let owned_animal: AnimalType = AnimalType::OwnedBy(owner_details);
    // Creating an `OwnedBy` variant. The `owner_details` struct instance
    // is moved into the enum variant.

    // -------------------------------------------------------------------------
    // 3. Using `match` Expressions: What it is and Use Cases
    // -------------------------------------------------------------------------
    // The `match` control flow operator is one of Rust's most powerful features.
    // It allows you to compare a value against a series of patterns and then
    // execute code based on which pattern the value matches.

    // Key characteristics of `match`:
    // - Exhaustiveness: Rust ensures that `match` expressions are exhaustive.
    //   This means you *must* cover every possible case for the type you are matching on.
    //   If you don't, the compiler will produce an error, preventing potential bugs.
    // - Pattern Matching: `match` allows for sophisticated pattern matching to
    //   destructure values, extract data, and bind parts of a value to variables.
    // - Control Flow: It's a powerful alternative to `if`/`else if`/`else` chains,
    //   especially when dealing with multiple distinct possibilities.

    println!("\n--- Match Use Case 1: Handling Enum Variants (Simple) ---");
    // When matching on enums without associated data, `match` acts like a
    // powerful switch statement.
    match fluffy {
        AnimalType::Dog => println!("Fluffy is a dog: Woof!"),
        AnimalType::Cat => println!("Fluffy is a cat: Meow!"),
        // The `_` (underscore) pattern acts as a wildcard or "catch-all" case.
        // It matches any value not matched by the preceding patterns. This is
        // often used to satisfy the exhaustiveness requirement when you don't
        // need specific logic for every single variant.
        _ => println!("Fluffy is an unknown animal type."),
    }

    let whiskers: AnimalType = AnimalType::Cat;
    match whiskers {
        AnimalType::Dog => println!("Whiskers is a dog (unexpected)."),
        AnimalType::Cat => println!("Whiskers is a cat: Purrrrfect! Meow!"),
        _ => println!("Whiskers is an unexpected animal type."),
    }

    println!("\n--- Match Use Case 2: Handling Enum Variants with Associated Data ---");
    // When enum variants carry data (like tuples or structs), `match` allows
    // you to destructure that data directly within the pattern.

    // Example of matching on an enum variant that holds a tuple:
    match chirpy {
        // We destructure the tuple directly into `species` and `age` variables.
        // These variables are then available for use within this `match` arm.
        AnimalType::Bird(species, age) => {
            println!("Chirpy is a bird: It's a {} aged {} years.", species, age);
        }
        _ => println!("Chirpy is not a bird (or not this variant)."),
    }

    // Example of matching on an enum variant that holds a struct (which itself contains a tuple):
    match owned_animal {
        // We destructure the `PetOwner` struct into its named fields (`name`, `age`).
        // We also destructure the `contact_info` tuple within the struct into `phone` and `email`.
        AnimalType::OwnedBy(PetOwner {
            name,
            age,
            contact_info: (phone, email),
        }) => {
            println!("This animal is owned by {} ({} years old).", name, age);
            println!("Owner's Contact: Phone: {}, Email: {}", phone, email);
        }
        _ => println!("This animal is not owned by anyone (or not this variant)."),
    }

    println!(
        "\n--- Match Use Case 3: Handling Different Numeric Values (Pattern Matching Features) ---"
    );
    // `match` is not just for enums; it's a general pattern matching construct.
    // You can use it with any type.

    let some_number = 7;
    match some_number {
        1 => println!("The number is exactly one."), // Matches a single literal value
        2 | 3 => println!("The number is two or three."), // Multiple patterns combined with `|` (OR)
        4..=6 => println!("The number is between four and six (inclusive)."), // Range pattern `start..=end`
        // `num @ 7..=10` is an "at" pattern. It matches the range 7 to 10,
        // and *additionally* binds the matched value to a new variable `num`.
        num @ 7..=10 => println!("The number is {} and it's between seven and ten.", num),
        _ => println!("The number is something else."), // Catch-all for any other value
    }

    // `if let` is a concise way to handle a single pattern match, where you
    // only care about one specific variant or pattern and ignore others.
    println!("\n--- `if let` Example (Concise Pattern Matching) ---");
    let another_number = 5;
    if let 4..=6 = another_number {
        // Checks if `another_number` matches the range 4 to 6
        println!("The number {} is in the range 4-6.", another_number);
    } else {
        println!("The number {} is outside the range 4-6.", another_number);
    }

    // `if let` is particularly common with `Option` and `Result` enums.
    enum Result<T, E> {
        // Simplified `Result` enum for demonstration
        Ok(T),
        Err(E),
    }

    let successful_result: Result<i32, String> = Result::Ok(100);
    let failed_result: Result<i32, String> = Result::Err(String::from("Operation failed"));

    if let Result::Ok(value) = successful_result {
        println!("Operation succeeded with value: {}", value);
    } else {
        println!("Operation failed.");
    }

    if let Result::Err(error_message) = failed_result {
        println!("Operation failed with error: {}", error_message);
    } else {
        println!("Operation succeeded (unexpectedly).");
    }

    // -------------------------------------------------------------------------
    // 4. Enums Can Carry Diverse Data (Beyond Simple Variants)
    // -------------------------------------------------------------------------
    // Enum variants are incredibly flexible; they can hold various data types,
    // including primitive types, tuples, and even other structs. This allows
    // you to represent complex, heterogeneous data in a type-safe manner.

    // This enum represents different geometric shapes. Each variant holds
    // the specific data required to define that shape.
    enum Shape {
        // `Circle` variant holds a named-field struct-like definition.
        // `radius` is a floating-point number, `center` is a tuple of two f64s.
        Circle { radius: f64, center: (f64, f64) },
        // `Rectangle` variant also holds a named-field struct-like definition.
        // `width` and `height` are floating-point numbers.
        Rectangle { width: f64, height: f64 },
        // `Triangle` variant holds a tuple of three f64s, representing side lengths.
        Triangle(f64, f64, f64),
    }

    let my_circle = Shape::Circle {
        radius: 5.0,
        center: (0.0, 0.0),
    };
    let my_rectangle = Shape::Rectangle {
        width: 10.0,
        height: 20.0,
    };
    let my_triangle = Shape::Triangle(3.0, 4.0, 5.0);

    fn describe_shape(shape: Shape) {
        println!("\n--- Describing a Shape ---");
        match shape {
            // Pattern matching to extract data from the `Circle` variant.
            Shape::Circle { radius, center } => {
                println!("Shape: Circle");
                println!("  Radius: {}", radius);
                println!("  Center: ({}, {})", center.0, center.1);
            }
            // Pattern matching to extract data from the `Rectangle` variant.
            Shape::Rectangle { width, height } => {
                println!("Shape: Rectangle");
                println!("  Width: {}", width);
                println!("  Height: {}", height);
            }
            // Pattern matching to extract data from the `Triangle` tuple variant.
            Shape::Triangle(s1, s2, s3) => {
                println!("Shape: Triangle");
                println!("  Sides: {}, {}, {}", s1, s2, s3);
            }
        }
    }

    describe_shape(my_circle);
    describe_shape(my_rectangle);
    describe_shape(my_triangle);

    // Another example: an `Event` enum to represent different types of user interactions.
    enum Event {
        Start,                    // Simple variant
        Click { x: i32, y: i32 }, // Variant with named fields (struct-like)
        KeyPress(char),           // Variant with a single value (tuple-like)
        Stop(u32),                // Variant with another single value
    }

    let click_event = Event::Click { x: 100, y: 200 };
    let key_event = Event::KeyPress('a');
    let stop_event = Event::Stop(123);

    fn handle_event(event: Event) {
        println!("\n--- Handling an Event ---");
        match event {
            Event::Start => println!("Event: Start"),
            Event::Click { x, y } => println!("Event: Click at ({}, {})", x, y),
            Event::KeyPress(key) => println!("Event: Key pressed: '{}'", key),
            Event::Stop(id) => println!("Event: Stop with ID: {}", id),
        }
    }

    handle_event(click_event);
    handle_event(key_event);
    handle_event(stop_event);

    // -------------------------------------------------------------------------
    // 5. Unnamed Associated Values (Tuple Variants)
    // -------------------------------------------------------------------------
    // Enum variants can hold unnamed associated values, which are essentially
    // tuples. These are useful when the order and type of the values are
    // sufficient to convey their meaning, without needing explicit field names.

    enum Coordinate {
        TwoD(f64, f64),        // Represents a 2D point (x, y)
        ThreeD(f64, f64, f64), // Represents a 3D point (x, y, z)
    }

    // Implementing methods for the `Coordinate` enum
    impl Coordinate {
        // Method to get the magnitude (distance from origin) for 2D or 3D coordinates
        fn magnitude(&self) -> f64 {
            match self {
                Coordinate::TwoD(x, y) => (x.powi(2) + y.powi(2)).sqrt(),
                Coordinate::ThreeD(x, y, z) => (x.powi(2) + y.powi(2) + z.powi(2)).sqrt(),
            }
        }

        // Method to get a string representation of the coordinate
        fn to_string(&self) -> String {
            match self {
                Coordinate::TwoD(x, y) => format!("({}, {})", x, y),
                Coordinate::ThreeD(x, y, z) => format!("({}, {}, {})", x, y, z),
            }
        }
    }

    let point2d = Coordinate::TwoD(3.0, 4.0);
    let point3d = Coordinate::ThreeD(1.0, 2.0, 2.0);

    fn print_coordinate(coord: Coordinate) {
        println!("\n--- Printing Coordinate ---");
        match coord {
            // Destructure the tuple values directly by their position.
            Coordinate::TwoD(x, y) => {
                println!("2D Coordinate: ({}, {})", x, y);
            }
            Coordinate::ThreeD(x, y, z) => {
                println!("3D Coordinate: ({}, {}, {})", x, y, z);
            }
        }
    }

    print_coordinate(point2d.clone()); // .clone() is used because `print_coordinate` takes ownership
    print_coordinate(point3d.clone()); // and we want to use the original `point2d`/`point3d` for methods below.

    println!(
        "Magnitude of 2D point {}: {}",
        point2d.to_string(),
        point2d.magnitude()
    );
    println!(
        "Magnitude of 3D point {}: {}",
        point3d.to_string(),
        point3d.magnitude()
    );

    // The `Bird` variant in `AnimalType` (e.g., `Bird(String, u32)`)
    // and the `Triangle` variant in `Shape` (e.g., `Triangle(f64, f64, f64)`)
    // are also examples of unnamed associated values.

    // -------------------------------------------------------------------------
    // 6. Enums with Methods
    // -------------------------------------------------------------------------
    // Just like structs, you can define methods on enums using an `impl` block.
    // The method's behavior can differ based on the enum variant, allowing for
    // polymorphic behavior.

    impl AnimalType {
        fn make_sound(&self) {
            println!("\n--- Making an Animal Sound ---");
            match self {
                AnimalType::Dog => println!("Woof! Woof!"),
                AnimalType::Cat => println!("Meow! Purr!"),
                // When matching `Bird`, we only care about the `species` for the sound.
                // The `_` ignores the `age` value in the tuple.
                AnimalType::Bird(species, _) => println!("A {} chirps!", species),
                // When matching `OwnedBy`, we destructure the `PetOwner` struct
                // to access the owner's `name` and `contact_info`.
                AnimalType::OwnedBy(owner_info) => {
                    println!(
                        "This animal is owned by {}. It might make a sound, but we don't know what kind!",
                        owner_info.name
                    );
                    println!("(Owner's email: {})", owner_info.contact_info.1); // Accessing tuple data within the struct
                }
                // Catch-all for any other future variants to ensure exhaustiveness.
                _ => println!("This animal makes an unknown sound!"),
            }
        }
    }

    fluffy.make_sound();
    whiskers.make_sound();
    // Note: `chirpy` and `owned_animal` were moved by `match` in section 3.
    // To use them here, they would need to be re-created or borrowed.
    // For simplicity, let's re-create them for this demonstration.
    let chirpy_recreated: AnimalType = AnimalType::Bird(String::from("Parrot"), 5);
    let owner_details_recreated = PetOwner {
        name: String::from("Alice"),
        age: 30,
        contact_info: (String::from("555-1234"), String::from("alice@example.com")),
    };
    let owned_animal_recreated: AnimalType = AnimalType::OwnedBy(owner_details_recreated);

    chirpy_recreated.make_sound();
    owned_animal_recreated.make_sound();
}
