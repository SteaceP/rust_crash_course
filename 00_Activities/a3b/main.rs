// Topic: Flow control using if..else if..else
//
// Program requirements:
// * Display ">5", "<5", or "=5" based on the value of a variable
//   is > 5, < 5, or == 5, respectively
//
// Notes:
// * Use a variable set to any integer value
// * Use an if..else if..else block to determine which message to display
// * Use the println macro to display messages to the terminal

// * Display ">5", "<5", or "=5" based on the value of a variable
fn main() {
    // * Use a variable set to any integer value
    let my_var = 5;
    // * Use an if..else if..else block to determine which message to display
    if (my_var) > 5 {
        println!("my variable is greater than 5");
    } else if (my_var) < 5 {
        println!("my variable is less than 5");
    } else {
        println!("my variable is equal to 5");
    }
}
