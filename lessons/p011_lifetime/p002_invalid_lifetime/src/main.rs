// This module demonstrates invalid lifetime usage in Rust
// Focus: Lifetime violations, dangling references, and borrow checker errors

/// Function that compares two string slices and returns the longer one
/// Same function as in the valid lifetime example
/// 
/// Lifetime annotation 'a:
/// - Both input parameters must have the same lifetime 'a  
/// - The return value also has lifetime 'a
/// - The returned reference cannot outlive the shortest input lifetime
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

/// Main function demonstrating invalid lifetime usage
/// This example shows what happens when lifetimes don't align properly
fn main() {
    // Create first string that lives for the entire main() function
    let string_1 = String::from("abcd");
    
    // Create a new scope block
    {
        // Create second string that only lives within this inner scope
        let string_2 = String::from("xyz");
        
        // Call longest function with references to both strings
        // At this point, both strings are alive, so this works
        let result = longest(&string_1, &string_2);
        
        // This print works because 'result' is used within the same scope
        // where both string_1 and string_2 are still alive
        println!("The longest string is {}", result); // This is valid
        
    } // string_2 is dropped here when it goes out of scope
    
    // COMPILATION ERROR: This line would cause a compile-time error
    // because 'result' might reference string_2, which has been dropped
    // The borrow checker prevents this dangling reference
    // println!("The longest string is {}", result); // This is invalid
    
    // To fix this, 'result' would need to be declared and used within
    // the same scope where all referenced data is still alive
}