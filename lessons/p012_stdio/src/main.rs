// This module demonstrates standard input/output operations in Rust
// Focus: Reading user input, string parsing, error handling with expect/unwrap

// Import the std::io module for input/output operations
use std::io;

/// Main function demonstrating console input and output
/// Shows how to read strings and numbers from standard input
fn main() {
    // === STRING INPUT EXAMPLE ===
    
    // Create a mutable String to store user input
    // String::new() creates an empty, growable string
    let mut string = String::new();
    
    // Display prompt to the user
    println!("Enter a string: ");
    
    // Read a line from standard input
    io::stdin()
        .read_line(&mut string)           // Pass mutable reference to store input
        .expect("Expected user input");   // Handle potential I/O errors
    
    // Display the entered string back to user
    // Note: read_line includes the newline character
    println!("You have entered this string: {}", string);

    // === NUMBER INPUT EXAMPLE ===
    
    // Create another mutable String for number input
    let mut line_input = String::new();
    
    // Display prompt for number input
    println!("Enter a number: ");
    
    // Read line from standard input (same pattern as above)
    io::stdin()
        .read_line(&mut line_input)       // Store input as string
        .expect("Expected user input");   // Handle I/O errors
    
    // Parse the string input into an integer
    // trim() removes whitespace including newline
    // parse::<i32>() converts string to 32-bit integer
    // unwrap() panics if parsing fails (not recommended for production)
    let number: i32 = line_input.trim().parse::<i32>().unwrap();
    
    // Display the parsed number
    println!("You have entered this number: {}", number)
}
