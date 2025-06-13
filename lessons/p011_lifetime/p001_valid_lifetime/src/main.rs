// This module demonstrates valid lifetime annotations in Rust
// Focus: Lifetime parameters, generic lifetimes, and borrow checker requirements

/// Function that compares two string slices and returns the longer one
/// This function requires lifetime annotations because it returns a reference
/// 
/// Lifetime annotation 'a:
/// - The lifetime parameter 'a tells Rust that the returned reference
///   will live at least as long as the shorter of the two input lifetimes
/// - Both input parameters must have the same lifetime 'a
/// - The return value also has lifetime 'a
/// 
/// Parameters:
/// - x: first string slice with lifetime 'a
/// - y: second string slice with lifetime 'a
/// 
/// Returns: reference to the longer string slice, also with lifetime 'a
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // Compare the lengths of both string slices
    if x.len() > y.len() {
        // Return the first string if it's longer
        x
    } else {
        // Return the second string if it's longer or equal
        y
    }
}

/// Main function demonstrating valid lifetime usage
/// This example works because both strings live long enough
fn main() {
    // Create two owned strings on the heap
    // Both strings will live for the entire duration of main()
    let string_1 = String::from("abcd");
    let string_2 = String::from("xyz");

    // Call longest function with references to our strings
    // This works because both string_1 and string_2 live long enough
    // The returned reference will be valid as long as both inputs are valid
    let result = longest(&string_1, &string_2);
    
    // Print the result - this is safe because 'result' references
    // data that is still alive (string_1 and string_2)
    println!("The longest string is {}", result);
    
    // Note: Both string_1 and string_2 are dropped here at the end of main()
    // but that's fine because 'result' is not used after this point
}