use std::println;

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!();

    println!("1 + 1 = {}", add(1, 1));
    println!("2 + 2 = {}", add(2, 2));
    println!("3 + 2 = {}", add(3, 2));

    let mut a = 0;
    loop {
        if (a > 5) {
            break;
        }

        println!("{:?}", a);

        a = a + 1;
    }

    println!();
}
