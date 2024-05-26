use std::mem;

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (int_param, bool_param) = pair;
    (bool_param, int_param)
}

fn main() {
    println!("Capter 2: Primitive");

    // Literal
    println!("1 - 2: {}", 1i32 - 2);
    println!("one million: {}", 1_000_000u16);

    // Tupels
    let a = ((2, 4, 6, 2), (3.5, "amin"), (true, 1.0, 5.0));
    println!("The value of a.0 is {:#?}", a.0);
    // println!("The value of a.1 is {a.1:#?}");
    // println!("The value of a.2 is {a.2:#?}");
    let b: (i32, bool) = (234, false);
    println!("reverse 234, false is {:?}", reverse(b));

    // Arrays
    let xs: [u16; 5] = [1, 4, 6, 3, 9];
    let big_array: [u8; 100] = [23; 100];

    println!("xs: {xs:?}");
    println!("xs: {big_array:?}");

    println!("size of array is: {:?}", mem::size_of_val(&xs));
    println!("slice array between 0 and 13: {:?}", &big_array[0..13]);
}
