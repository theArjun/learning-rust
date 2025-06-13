// This module demonstrates module system and organization in Rust
// Focus: Module definition, public/private visibility, use statements, and code organization

/// A module containing mathematical operations
/// Modules help organize code into logical groups and control visibility
pub mod my_modules {
    /// Public function to subtract two integers
    /// The 'pub' keyword makes this function accessible from outside the module
    /// 
    /// Parameters:
    /// - a: the minuend (number to subtract from)
    /// - b: the subtrahend (number to subtract)
    /// 
    /// Returns: the difference (a - b)
    pub fn subtract(a: i32, b: i32) -> i32 {
        // Perform subtraction and return result
        a - b
    }
}

// Import the subtract function from my_modules into the current scope
// This allows us to call subtract() directly instead of my_modules::subtract()
use my_modules::subtract;

/// Main function demonstrating module usage and imports
fn main() {
    // Initialize test values
    let a = 5;
    let b = 10;
    
    // Call the imported subtract function
    // Because we imported it with 'use', we can call it directly
    let result = subtract(a, b);
    
    // Display the calculation and result
    // Note: The result will be negative since a < b
    println!("The result of {} - {} is {}", a, b, result);
}
