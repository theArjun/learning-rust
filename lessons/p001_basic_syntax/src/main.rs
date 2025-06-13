// This module demonstrates the basic syntax of Rust programming language
// Focus: Variable declaration, mutability, and basic output

/// Function to demonstrate variable declarations and mutability in Rust
/// This function shows the difference between immutable and mutable variables
pub fn learn_variables() {
    // Variables in Rust are immutable by default. Use mut to make them mutable:
    
    // Immutable variable - cannot be changed after initialization
    let x = 5;
    
    // Mutable variable - can be changed after initialization
    let mut y = 10;
    
    // Changing the value of a mutable variable
    y = 15;

    // Using println! macro to display variable values
    // {} is a placeholder for the variable value
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

/// Main function - entry point of the program
/// This function is called when the program starts execution
fn main() {
    // Call the function to demonstrate basic variable usage
    learn_variables()
}
