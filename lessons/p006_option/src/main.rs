// This module demonstrates the Option enum in Rust
// Focus: Option<T>, Some/None variants, pattern matching with nullable values

/// Function that returns an Option type to handle potential absence of values
/// Option<T> is Rust's way of representing nullable values safely
/// 
/// Parameters:
/// - id: a 32-bit unsigned integer to search for
/// 
/// Returns: Option<String> - either Some(String) if found, or None if not found
pub fn find_one(id: u32) -> Option<String> {
    // Conditional logic to determine return value
    if id == 1 {
        // Some wraps a value when we have something to return
        // String::from creates a String from a string literal
        Some(String::from("1 found."))
    } else {
        // None represents the absence of a value
        // This is Rust's safe alternative to null pointers
        None
    }
}

// Usage example demonstrating how to work with Option types:

/// Main function demonstrating Option handling and pattern matching
fn main() {
    // Test value for the function call
    let num = 2;
    
    // Call the function that returns an Option<String>
    let result = find_one(num);
    
    // match expression to handle both Some and None cases
    // This is the safe way to work with potentially absent values
    match result {
        // If result contains a value (Some variant)
        // Extract the value and bind it to the variable 'value'
        Some(value) => println!("{}", value),
        
        // If result is empty (None variant)
        // Handle the absence of a value gracefully
        None => println!("None")
    }
}

