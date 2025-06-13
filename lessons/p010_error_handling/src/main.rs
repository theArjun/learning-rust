// This module demonstrates error handling in Rust using the Result type
// Focus: Result<T, E>, Ok/Err variants, error handling patterns, and best practices

// The Result Type
// Used for functions that might fail:
// Result<T, E> represents either success (Ok(T)) or failure (Err(E))

/// Function that demonstrates error handling with Result type
/// Division operation that can fail when divisor is zero
/// 
/// Parameters:
/// - number: the dividend (number to be divided)
/// - divisor: the divisor (number to divide by)
/// 
/// Returns: Result<i32, String> - Ok(quotient) on success, Err(message) on failure
pub fn divide(number: i32, divisor: i32) -> Result<i32, String> {
    // Check for division by zero error condition
    if divisor == 0 {
        // Return an error with a descriptive message
        // Err variant contains the error information
        return Err("Cannot divide by zero".to_string());
    }
    
    // If divisor is not zero, perform the division
    // Ok variant contains the successful result
    Ok(number / divisor)
}

/// Main function demonstrating different ways to handle Result types
fn main() {
    // Test values for division operation
    let number = 40;
    let divisor = 0; // This will cause an error

    // Call the function that returns a Result
    let result = divide(number, divisor);

    // Way 1: Use unwrap to get the value or panic if it fails; can panic.
    // unwrap() will panic (crash your program) if the result is an Err.
    // COMMENTED OUT because it would crash the program:
    // let value = result.unwrap();
    // println!("The result is {}", value);

    // Way 2: Use match - Most explicit and safe approach
    // This handles both success and error cases explicitly
    match result {
        // If the operation succeeded, print the result
        Ok(value) => println!("Division successful: {}", value),
        
        // If the operation failed, print the error message
        Err(e) => println!("Division failed: {}", e),
    }

    // Way 3: Using if let - Concise when you mainly care about success
    // This is useful when you only want to handle the success case
    // EXAMPLE (commented out to avoid confusion with variable names):
    // if let Ok(quotient) = divide(number, 2) {
    //     println!("Success: {}", quotient);
    // } else {
    //     println!("Failed to divide {} by 2", number);
    // }
}

// Summary of Error Handling Approaches:
// match: Best for handling both success and error explicitly.
// if let: Great for a concise check when you mainly care about the successful case.
// unwrap() / expect(): Handy for prototyping or when you're sure an error won't occur, but be cautious!
