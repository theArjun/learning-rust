// This module demonstrates Rust's ownership and borrowing system
// Focus: Ownership rules, borrowing, references, dereferencing, and string slices

// What is Ownership?
// Rust's ownership system manages memory. Each value has a single owner, and when the owner goes out of scope, the value is dropped automatically.

// Borrowing and References
// You can borrow references to values using & (immutable) or &mut (mutable):

// These ampersands represent references, and they allow you to refer to some value
// without taking ownership of it [i.e. borrowing].

// The opposite of referencing by using & is dereferencing,
// which is accomplished with the dereference operator, *.

/// Function that borrows a reference to a String to calculate its length
/// This function takes a reference (&String) instead of ownership
/// This allows the caller to keep using the String after the function call
/// 
/// Parameters:
/// - s: an immutable reference to a String
/// 
/// Returns: the length of the string as usize
pub fn calculate_length(s: &String) -> usize {
    // Call the len() method on the borrowed String
    // We can read from the reference but cannot modify it
    s.len() // Returns the length without taking ownership
}

// Usage example (commented out):
//     let s1= String::from("hello");
//     let length = ownership_borrowing::calculate_length(&s1);
//     println!("The length of '{}' is {}.", s1, length);

/// Function demonstrating references and dereferencing
/// Shows how to create references and access their values
/// 
/// Returns: true if all assertions pass
pub fn check_referencing() -> bool {
    // Create a value on the stack
    let x = 5;
    
    // Create a reference to x using the & operator
    // y now points to the memory location of x
    let y = &x;

    // Direct comparison: x equals 5
    assert_eq!(x, 5);
    
    // Dereference y using * to get the value it points to
    // *y gives us the value that y references (which is x's value)
    assert_eq!(*y, 5);

    // All assertions passed
    true
}

/// Function that works with string slices instead of owned Strings
/// String slices (&str) are more flexible as they work with both
/// String objects and string literals
/// 
/// Parameters:
/// - s: a string slice reference
/// 
/// Returns: the length of the string slice
pub fn calculate_substring_length(s: &str) -> usize {
    // String slices also have the len() method
    s.len()
}

/// Main function demonstrating string slicing and borrowing
fn main() {
    // Create an owned String on the heap
    let s = String::from("hello");
    
    // Create a string slice from part of the String
    // &s[0..3] creates a slice containing the first 3 characters
    // This is a reference to part of the original string
    let slice = &s[0..3]; // "hel"
    
    // Pass the slice to our function
    // String slices can be passed to functions expecting &str
    let length = calculate_substring_length(&slice);
    
    // Print the slice and its length
    // Both 'slice' and 's' are still valid because we only borrowed
    println!("The length of {} is {}", slice, length);
}