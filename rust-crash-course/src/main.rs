// #![deny(clippy::all)]
// #[derive(PartialEq)]
// #[derive(Debug)]

fn main() {
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
}
