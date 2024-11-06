// Basic subtraction
fn subtract(a: i32, b: i32) -> i32 {
    return a - b;
}

fn main() {
    // Basic arithmetic
    let sum = 2 + 2; //4
    let value = 10 - 5; // 5
    let division = 10 / 2; // 5
    let multiplication = 5 * 5; // 25

    let five = subtract(8, 3); // 5

    // Modulo
    let modulo = 6 % 3; // 0
    let modulo2 = 6 % 4; // 2

    // print values
    println!("Sum: {}", sum);
    println!("Value: {}", value);
    println!("Division: {}", division);
    println!("Multiplication: {}", multiplication);
    println!("Subtraction: {}", five);
    println!("Modulo: {}", modulo);
    println!("Modulo2: {}", modulo2);
}
