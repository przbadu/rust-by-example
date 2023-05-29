// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct GroceryItem {
    id: i32,
    quantity: i32,
}

// dont Transfer Ownership, of GroceryItem
fn display_quantity(item: &GroceryItem) {
    println!("Quantity: {:?}", item.quantity)
}

// dont Transfer Ownership, of GroceryItem
fn display_id(item: &GroceryItem) {
    println!("Id: {:?}", item.id)
}

fn main() {
    let item = GroceryItem {
        id: 1,
        quantity: 10,
    };

    // Borrow item
    display_quantity(&item);
    // Borrow item
    display_id(&item);
} // item will be moved from memory here.
