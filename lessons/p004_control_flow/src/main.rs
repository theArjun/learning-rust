// This module demonstrates control flow structures in Rust
// Focus: if/else statements, for loops, while loops, and conditional logic

// Use if, else, and loops (loop, while, for) to control program flow:

/// Function to demonstrate basic control flow structures
/// Shows if/else statements and for loops
pub fn learn_control_flow() {
    // Initialize a variable for demonstration
    let x = 10;

    // if/else statement: executes different code blocks based on conditions
    if x > 5 {
        println!("x is greater than 5");
    } else {
        println!("x is 5 or less");
    }

    // for loop: iterates over a range of numbers
    // 0..5 creates a range from 0 (inclusive) to 5 (exclusive)
    for i in 0..5 {
        println!("{}", i);
    }
}

/// Function to check if a number is odd or even
/// Demonstrates modulo operator and conditional logic
pub fn is_number_odd() {
    // Test number for odd/even check
    let x = 23;
    
    // Use modulo operator (%) to check for even/odd
    // If x % 2 equals 0, the number is even; otherwise, it's odd
    if x % 2 == 0 {
        println!("x is even");
    } else {
        println!("x is odd");
    }
}

/// Function to check if a number is prime
/// 
/// Parameters:
/// - num: the integer to check for primality
/// 
/// Returns: true if the number is prime, false otherwise
/// 
/// Algorithm: Check if the number is divisible by any integer from 1 to num/2
pub fn is_number_prime(num: i32) -> bool {
    // Start checking from 1 (we skip 0 to avoid division by zero)
    // If we initialize with 0 here, we need to handle case explicitly.
    let mut i = 1;
    
    // while loop: continues as long as the condition is true
    // We only need to check up to num/2 since larger divisors
    // would have corresponding smaller divisors
    while i <= num / 2 {
        // If num is divisible by i, it's not prime
        if num % i == 0 {
            return false; // Early return if divisor found
        }
        i += 1; // Increment counter for next iteration
    }
    
    // If no divisors found, the number is prime
    true
}

/// Main function - entry point demonstrating all control flow examples
fn main() {
    // Demonstrate basic control flow structures
    learn_control_flow();
    
    // Demonstrate odd/even checking
    is_number_odd();

    // Test prime number checking
    let num = 29;
    
    // Use the return value of is_number_prime in an if statement
    if is_number_prime(num) {
        println!("{} is a prime number", num);
    } else {
        println!("{} is not a prime number", num);
    }
}