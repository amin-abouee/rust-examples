fn main() {
    // Create an empty vector of i32
    let vec_1: Vec<i32> = Vec::new();
    println!("vec_1 (empty): {:?}", vec_1);

    // Create a vector with initial values
    let vec_2: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("vec_2 (initialized): {:?}", vec_2);

    // Demonstrate different ways to access elements
    let fifth_element: &i32 = &vec_2[4]; // Direct indexing (panics if out of bounds)
    let fifth_element_get: Option<&i32> = vec_2.get(4); // Safe access method

    println!("Fifth element (direct access): {:?}", fifth_element);
    println!("Fifth element (safe access): {:?}", fifth_element_get);

    // Demonstrate creating and modifying a mutable vector
    let mut vec_3 = Vec::new();
    vec_3.push(1);
    vec_3.push(2);
    vec_3.push(3);
    println!("vec_3 (after pushing elements): {:?}", vec_3);

    // Demonstrate iterating over a vector
    println!("Iterating over vec_3:");
    for (index, value) in vec_3.iter().enumerate() {
        println!("Element {} at index {}", value, index);
    }

    // Demonstrate some additional vector operations
    vec_3.pop(); // Remove the last element
    println!("vec_3 (after pop): {:?}", vec_3);

    vec_3.insert(1, 10); // Insert 10 at index 1
    println!("vec_3 (after insert): {:?}", vec_3);

    vec_3.remove(0); // Remove element at index 0
    println!("vec_3 (after remove): {:?}", vec_3);

    // Demonstrate vector capacity
    println!("vec_3 length: {}, capacity: {}", vec_3.len(), vec_3.capacity());

    // Define an enum to hold different numeric types
    #[derive(Debug)]
    enum Number {
        Integer32(i32),
        Integer64(i64),
        Float32(f32),
        Float64(f64),
    }

    // Create a vector of Numbers
    let mixed_vec: Vec<Number> = vec![
        Number::Integer32(42),
        Number::Integer64(1234567890123456789),
        Number::Float32(3.14),
        Number::Float64(2.71828182845904523536),
    ];

    println!("Mixed vector: {:?}", mixed_vec);

    // Iterate over the vector and print each element
    for num in &mixed_vec {
        match num {
            Number::Integer32(n) => println!("i32: {}", n),
            Number::Integer64(n) => println!("i64: {}", n),
            Number::Float32(n) => println!("f32: {}", n),
            Number::Float64(n) => println!("f64: {}", n),
        }
    }

    // Example of three strings
    let string1 = String::from("Hello");
    let string2 = String::from(", ");
    let string3 = String::from("World!");

    // Combining the strings using format!
    let combined_string = format!("{}{}{}", string1, string2, string3);

    println!("Combined string: {}", combined_string);
}
