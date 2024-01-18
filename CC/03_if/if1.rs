// if1.rs
//
// Execute `rustlings hint if1` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

pub fn bigger(a: i32, b: i32) -> i32 {
    // Complete this function to return the bigger number!
    // Do not use:
    // - another function call
    // - additional variables
    if a > b { a } else { b }

}

fn main() {
    // Add your code here to test the `bigger` function
    println!("The bigger number is: {}", bigger(10, 8));
    println!("The bigger number is: {}", bigger(32, 42));
    println!("The bigger number is: {}", bigger(42, 42));
}
