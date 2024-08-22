use std::{collections::HashMap, fs::File};

fn main() {
    let mut hash: HashMap<String, i32> = HashMap::new();
    hash.insert(String::from("ir"), 2);
    hash.insert(String::from("de"), 17);
    println!("{:?}", hash);

    for (key, value) in &hash {
        println!("Key: {key} -> Value: {value}");
    }
    // Check if a key exists
    if hash.contains_key("de") {
        println!("'de' exists in the hash map");
    }

    // Get a value from a key
    match hash.get("de") {
        Some(value) => {
            println!("Value for 'de': {}", value);
            println!("The 'match' expression is evaluating the result of hash.get(\"de\")");
            println!("'Some(value)' means the key 'de' was found, and 'value' contains its associated value");
            println!(
                "'Some' is an enum variant from the Option type, indicating presence of a value"
            );
        }
        None => {
            println!("'de' not found in the hash map");
            println!("'None' is returned when the key doesn't exist in the HashMap");
        }
    }

    // Another way to get a value (panics if key doesn't exist)
    let value_de: i32 = hash["de"];
    println!("Value for 'de': {}", value_de);

    let text: String =
        String::from("I like Rust programming languages , Rust is a good programming languages");
    let mut w_hash = HashMap::new();

    for word in text.split_whitespace() {
        let count = w_hash.entry(word).or_insert(0);
        *count += 1;
    }

    for (k, v) in &w_hash {
        println!("Word: {k} -> Value: {v}");
    }

    // let filename: Result<File, std::io::Error> = File::open("input.txt");
    // let output_file = match filename {
    //     Ok(file) => file,
    //     Err(error) => panic!("File doesn't exist: {:?}", error),
    // };

    let f1 = File::open("ou.txt").expect("File doesn't exist");
}
