// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
v
// * Use a function to add two numbers together
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

// * Use a function to display the result
fn display_result() {
    // * Use the "{:?}" token in the println macro to display the result
    println!("result: {:?},", add(2, 5))
}

fn main() {
    display_result();
}
