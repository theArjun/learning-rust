// This module demonstrates Foreign Function Interface (FFI) in Rust
// Focus: Creating Rust functions that can be called from C/C++ or other languages

/// Function to add two 32-bit integers
/// This function is designed to be called from C or other foreign languages
/// 
/// Attributes:
/// - #[no_mangle]: Prevents Rust from changing the function name during compilation
///   This ensures the function can be found by its exact name from other languages
/// - pub extern "C": Makes the function public and use C calling conventions
///   The "C" ABI (Application Binary Interface) is compatible with most languages
/// 
/// Parameters:
/// - a: first integer to add
/// - b: second integer to add
/// 
/// Returns: sum of a and b
#[no_mangle]
pub extern "C" fn add_numbers(a: i32, b: i32) -> i32 {
    // Simple addition operation
    // This will be accessible from C as: int add_numbers(int a, int b);
    a + b
}

/// Function to subtract two 32-bit integers
/// Similar to add_numbers but performs subtraction
/// 
/// Attributes:
/// - #[no_mangle]: Preserves function name for foreign language access
/// - pub extern "C": Public function with C calling conventions
/// 
/// Parameters:
/// - a: number to subtract from (minuend)
/// - b: number to subtract (subtrahend)
/// 
/// Returns: difference of a - b
#[no_mangle]
pub extern "C" fn subtract_numbers(a: i32, b: i32) -> i32 {
    // Simple subtraction operation
    // This will be accessible from C as: int subtract_numbers(int a, int b);
    a - b
}
