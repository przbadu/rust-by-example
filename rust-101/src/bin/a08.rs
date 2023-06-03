// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

use std::println;

#[derive(Debug)]
enum Flavors {
    Lemon,
    Martini,
}

struct Drink {
    flavor: Flavors,
    fluid_oz: f64,
}

fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavors::Martini => println!("Flavor: Martini"),
        Flavors::Lemon => println!("Flavor: Lemon"),
    }
    println!("Fluid Oz: {:?}", drink.fluid_oz);
}

fn main() {
    let lemonade = Drink {
        flavor: Flavors::Lemon,
        fluid_oz: 2.3,
    };
    print_drink(lemonade);

    let martini = Drink {
        flavor: Flavors::Martini,
        fluid_oz: 10.0,
    };
    print_drink(martini);
}
