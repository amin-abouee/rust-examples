// This is the first rust file

/*
    This code copied from https://doc.rust-lang.org/rust-by-example/hello/comment.html
*/

fn main() {
    println!("Hello, world!");

    let x = 50 + 5;
    println!("The output of 50 + 5 is {}", x);

    // Different formatting can be invoked by specifying the format character
    // after a `:`.
    println!("Base 10:               {}",   69420); // 69420
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c

    println!("{0} is my name and {1} is my family, {1},{0}", "Amin", "Abouee");

    println!("{number:0>6}", number=13);
    println!("{number:0<6}", number=13);

    let f: f64 = 54.9;
    let n: usize = 6;
    println!("{f}:>{n}");
    println!("{f:0>n$}");
}
