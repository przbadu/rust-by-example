use std::println;

// Globals are declared outside all other scopes.
static LANGUAGE: &str = "Rust";
const THREASHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // Access constant in some function
    n > THREASHOLD
}

fn main() {
    let n = 16;

    // Access constant in the main thread
    println!("This is {}", LANGUAGE);
    println!("The threashold is {}", THREASHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // THREASHOLD = 5; // ERR
}
