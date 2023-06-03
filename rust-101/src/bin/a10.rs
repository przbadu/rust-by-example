// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

use std::println;

fn print_number(num: i32) {
    let result = if num <= 100 { "<=100" } else { ">100" };

    println!("{:?}", result);
}

fn main() {
    print_number(20);
}
