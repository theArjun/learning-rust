// This module demonstrates function definition and usage in Rust
// Focus: Function syntax, parameters, return types, and function calls

/// A simple function that adds two 32-bit integers
/// 
/// Parameters:
/// - a: first integer (i32)
/// - b: second integer (i32)
/// 
/// Returns: sum of a and b (i32)
/// 
/// The last expression in a function is automatically returned (no semicolon needed)
pub fn add(a: i32, b: i32) -> i32 {
    // Return the sum of a and b
    // Note: No semicolon here because this is the return expression
    a + b
}

/// Main function - entry point of the program
/// Demonstrates calling a function and using its return value
fn main() {
    // Call the add function with arguments 1 and 2
    // Store the returned result in a variable
    let result = add(1, 2);
    
    // Print the result using println! macro
    // {} is a placeholder that will be replaced with the result value
    println!("{}", result);
}
