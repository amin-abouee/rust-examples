struct Person {
    Name: String,
    Age: i32,
}

enum Nums {
    X,
    Y,
    Z,
}

struct User {
    name: String,
    mail: String,
    is_active: bool,
    age: u8,
}

// unit structure
struct Pt;

// normal struct
struct Point2d {
    width: u32,
    height: u32,
}

//tuple struct
struct Point3d(u32, u32, u32);

enum Point56 {
    Pt1,
    Pt2 { x: i32, y: i32 },
    Pt3(i32, i32),
}

struct Point {
    width: u32,
    height: u32,
}

impl Point {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // associate function
    fn contor(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

enum Coin {
    Nikel,
    Mass(Reality),
    Gold,
}

enum Reality {
    Epic,
    Common,
    Rare,
}

fn another_function() {
    println!("another function");
}

fn main() {
    let enum_coin = Coin::Mass(Reality::Epic);

    if let Coin::Nikel = enum_coin {
        println!("This is a nikel coin");
    } else {
        println!("This is not a nikel coin")
    }

    println!("Hello, world!");
    let x: i32 = 16;
    println!("the value of x is {x}");

    let x: &str = "bingo bingo ";
    println!("the value of x is {x}");

    const Y: i128 = 120_000_000;
    println!("the value of y is {Y}");

    let z: f64 = 34.0;
    let u: i32 = 143;

    println!("value of {z} and {u}");

    let tpl: (i32, u64, f32) = (123, 543653, 234.09);
    println!(
        "value of the new tuple is {:#?} and the first element of that is {}",
        tpl, tpl.0
    );

    let _arr1: [i32; 50] = [23; 50];
    // println!("arr1 is: {:#?}", arr1);

    // for int in arr1.iter()
    // {
    //     println!("elemnt are {int}");
    // }

    let _per: Person = Person {
        Name: "amin".to_string(),
        Age: 38,
    };

    let jh = Nums::X;

    match jh {
        Nums::X => println!("X"),
        Nums::Y => println!("Y"),
        Nums::Z => println!("Z"),
    }

    another_function();

    let tmp_x = {
        let r = 3.12;
        r + 2f64
    };
    if tmp_x < 5.0 {
        println!("number is less than 5");
    } else {
        println!("number is more than 5");
    }

    let mut y = 0;
    loop {
        y += 1;
        println!("y is {y}");
        if y == 10 {
            break;
        }
    }

    for number in 1..20 {
        println!("for loop, {number}");
    }

    for i in 1..=100 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else {
            println!("{i}");
        }
    }

    let ui: String = String::from("hello world");

    let re = &ui;

    println!("{re}");
    println!("{ui}");

    let io = 234;
    let rt = io;
    println!("{rt}");

    let s1: String = String::from("Hello world");
    let sz: usize = test_borrowing(&s1);
    println!("String {s1} has {sz} size.");

    let sn = String::from("bingo");
    let s2: &String = &sn;
    let s3: &String = &sn;

    println!("s2 id {s2} and s3 is {s3}");

    let str_arr: [char; 5] = ['a', 'b', 'c', 'd', 'e'];
    let slice: &[char] = &str_arr[1..=3];
    println!("slice is {:?}", slice);

    let vec1: Vec<i32> = vec![123, 44, 345, 324];
    let vec_slice: &[i32] = &vec1[1..3];

    let new_user = User {
        name: String::from("amin"),
        mail: String::from("check@gmail.com"),
        age: 38,
        is_active: true,
    };

    println!("age amin is {}", new_user.age);

    let e1: Option<i32> = Some(120);
    let e2: i32 = 34;
    println!("sum: {}", e1.unwrap() + e2);

    let config_max: Option<i32> = Some(100);
    match config_max {
        Some(max) => println!("The max is: {}", max),
        _ => (),
    }
}

fn test_borrowing(s: &String) -> usize {
    s.len()
}

fn dangle() -> String {
    let s = String::from("test");
    s
}
