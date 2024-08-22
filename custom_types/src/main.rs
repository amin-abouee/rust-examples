#![allow(dead_code)]
#[derive(Debug)]

struct Person {
    name: String,
    age: u8,
}

struct Pair(f32, f32);

struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn area(&self) -> f32 {
        let Rectangle {
            top_left: Point { x: x1, y: y1 },
            bottom_right: Point { x: x2, y: y2 },
        } = self;
        let width: f32 = (x2 - x1).abs();
        let height: f32 = (y2 - y1).abs();
        width * height
    }
}

enum Status {
    Rich,
    Poor,
}

fn main() {
    println!("Chapter3: Custom Types");

    let amin = Person {
        name: "Amin".to_string(),
        age: 35,
    };

    println!("amin variable is {amin:?}");

    let rect = Rectangle {
        top_left: Point { x: 0f32, y: 0f32 },
        bottom_right: Point {
            x: 3.4f32,
            y: 5.3f32,
        },
    };
    println!("The area is {:?}", rect.area());

    let sample: Status = Status::Poor;
    println!("sample status is {:?}", sample as i8);
}
