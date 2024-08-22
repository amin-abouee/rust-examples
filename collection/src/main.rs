fn main() {
    let vec_1: Vec<i32> = Vec::new();
    println!("vec_1: {:?}", vec_1);

    let vec_2: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("vec_2: {:?}", vec_2);

    let fifth_element: &i32 = &vec_2[4];
    let fifth_element_get: Option<&i32> = vec_2.get(4);

    println!("fifth_element: {:?}", fifth_element);
    println!("fifth_element_get: {:?}", fifth_element_get);

    let mut vec_3 = Vec::new();
    vec_3.push(1);
    vec_3.push(2);
    vec_3.push(3);
    println!("vec_3: {:?}", vec_3);

    for i in &vec_3 {
        println!("{i}");
    }

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
