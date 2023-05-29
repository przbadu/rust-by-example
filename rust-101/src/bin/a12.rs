// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

enum Color {
    Red,
    Green,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Red => println!("red"),
            Color::Green => println!("green"),
        }
    }
}

struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

impl Dimensions {
    fn print(&self) {
        println!("width: {:?}", self.width);
        println!("height: {:?}", self.height);
        println!("depth: {:?}", self.depth);
    }
}

struct ShippingBox {
    dimensions: Dimensions,
    weight: f32,
    color: Color,
}

impl ShippingBox {
    fn new(weight: f32, color: Color, dimensions: Dimensions) -> Self {
        Self {
            dimensions,
            weight,
            color,
        }
    }

    fn print(&self) {
        println!("");
        println!("Weight: {:?}", self.weight);
        self.dimensions.print();
        self.color.print();
    }
}

fn main() {
    let dimensions = Dimensions {
        width: 24.0,
        height: 20.0,
        depth: 32.0,
    };
    let red_box = ShippingBox::new(10.0, Color::Red, dimensions);
    red_box.print();

    let dimensions = Dimensions {
        width: 10.0,
        height: 20.0,
        depth: 15.0,
    };
    let green_box = ShippingBox::new(20.0, Color::Green, dimensions);
    green_box.print();
}
