// An attribute to hide warning for unused code.
#![allow(dead_code)]

use std::println;

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A struct with two fields
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

// fn rect_area(rect: Rectangle) -> f32 {

// }

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 21;
    let peter = Person { name, age };

    println!("");

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields
    // or our other one
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we
    // used that field from `point`
    println!("Second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let _rectangle = Rectangle {
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);

    // Activity:
    // 1. Add afnction `rect_area` which calculates the area of
    // a `Rectangle` (try using nested destructuring)
    println!();

    // 2. Add a function `square` which takes a `Point` and a `f32`
    // as arguments, and returns a `Rectangle` with its top left corner
    // on the point, and a width and height corrosponding to the `f32`.

    println!("");
}
