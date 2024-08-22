fn main() {
    println!("Chapter 5: Types");

    let decimal: f32 = 45.23566_f32;
    let integer: u32 = decimal as u32;

    println!("integer values: {integer}");

    // Error! There are limitations in conversion rules.
    // A float cannot be directly converted to a char.

    println!("nan as u8 is : {}", f32::NAN as u8);
    println!("maximum u32 is : {}", u32::max_value());
    println!("maximum f64 {}, minimum f64 {}", f64::MAX, f64::MIN);
    println!("max is {}", u128::MAX)
}
