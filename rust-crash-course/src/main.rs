#![deny(clippy::all)]
#[derive(PartialEq)]
struct PetOwner {
    name: String,
    age: u32,
    contact_info: (String, String), // Tuple for (phone_number, email)
}

fn main() {
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
}
