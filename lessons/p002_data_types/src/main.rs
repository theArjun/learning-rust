// This module demonstrates different data types available in Rust
// Focus: Scalar types (integers, floats, booleans, characters) and compound types (tuples, arrays)

// Rust has scalar types (integers like i32, floats like f64, booleans, characters) and compound types (tuples, arrays):

/// Function to demonstrate various data types in Rust
/// Shows examples of tuples and arrays with their type annotations
pub fn learn_data_types() {
    // Tuple: A compound type that groups multiple values of different types
    // Type annotation: (i32, f64, u8) specifies the types of each element
    // Values: (500, 6.4, 1) - integer, float, unsigned 8-bit integer
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    
    // Array: A collection of elements of the same type with fixed length
    // Type annotation: [i32; 4] means array of 4 i32 integers
    // All elements must be of the same type (i32 in this case)
    let arr: [i32; 4] = [1, 2, 3, 4];

    // Print the tuple using debug formatting {:?}
    // Debug formatting allows printing of complex data structures
    println!("{:?}", tup);
    
    // Print the array using debug formatting {:?}
    println!("{:?}", arr);
}

/// Main function - entry point of the program
/// Calls the data types demonstration function
fn main() {
    // Execute the data types learning function
    learn_data_types();
}
