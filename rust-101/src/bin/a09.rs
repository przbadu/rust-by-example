// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

fn coords(x: i32, y: i32) -> (i32, i32) {
    (x, y)
}

fn main() {
    let (_x, y) = coords(10, 20);

    match y.cmp(&5) {
        std::cmp::Ordering::Less => println!("<5"),
        std::cmp::Ordering::Equal => println!("=5"),
        std::cmp::Ordering::Greater => println!(">5"),
    }
}
