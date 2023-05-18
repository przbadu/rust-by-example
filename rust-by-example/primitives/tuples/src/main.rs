use std::fmt;

// The following struct is for the activity.
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

// Tuples can be used as a function arguments and as return values.
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind the members of a tuple to variables.
    let (int_params, bool_param) = pair;

    (bool_param, int_params)
}

fn transpose(mat: Matrix) -> Matrix {
    let (a, b) = (mat.2, mat.1);

    Matrix(mat.0, a, b, mat.3)
}

fn main() {
    println!();

    // A tuple with a bunch of different types.
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );

    // Values can be extracted from the tuple using tuple indexing.
    println!("Long tuple first value: {}", long_tuple.0);
    println!("Long tuple second value: {}", long_tuple.1);

    // Tuples can be tuple members.
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // Tuples are printable
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // But long Tuples (more than 12 elements) cannot be printed.
    //let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    //println!("Too long tuple: {:?}", too_long_tuple);
    // TODO ^ Uncomment the above 2 lines to see the compiler error

    let pair = (1, true);
    println!("Pair is {:?}", pair);

    println!("The reversed pair is {:?}", reverse(pair));

    // To create one element tuples, the comma is required to tell them apart
    // from a literal surrounded by parantheses.
    println!("One element tuple: {:?}", (5u32,));
    println!("Just an integer: {:?}", (5u32));

    // Tuples can be destructured to create bindings.
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
    println!();

    // Activity 1:
    // Recap: Add the `fmt::Display` trait to the Matrix struct in the above example, so that if
    // you switch from printing the debug format `{:?}` to the display format `{}`, you see the
    // following output:
    // ( 1.1 1.2 )
    // ( 2.1 2.2 )

    impl fmt::Display for Matrix {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
        }
    }

    println!("{}", matrix);
    println!("");

    // 2. Add a `transpose` function using the reverse function as a template, which accepts a
    //    matrix as an argument, and returns a matrix in which two elements have been swapped. for
    //    example:
    //    ```rust
    //    println!("Matrix:\n{}", matrix);
    //    println!("Transpose:\n{}", transpose(matrix));
    //    ```
    //    Results in the output:
    //    ```rust
    //    Matrix:
    //    ( 1.1 1.2 )
    //    ( 2.1 2.2 )
    //    Transpose:
    //    ( 1.1 2.1 )
    //    ( 1.2 2.2 )
    //    ```
    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose(matrix));

    println!();
}
