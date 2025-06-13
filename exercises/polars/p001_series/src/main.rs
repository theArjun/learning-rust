// This module demonstrates basic usage of Polars Series
// Focus: Creating and displaying Series (one-dimensional labeled arrays)

// Import the Polars prelude which includes commonly used types and traits
use polars::prelude::*;
// Explicitly import the Series type for clarity
use polars::series::Series;

/// Function to demonstrate Series creation and basic operations
/// A Series is similar to a column in a DataFrame or a pandas Series
pub fn learn_series() {
    // Create a new Series with integer data
    // Series::new() takes two parameters:
    // 1. Name/label for the series (converted to string with .into())
    // 2. Data as a slice reference (&[1, 2, 3, 4, 5])
    let s = Series::new("ints".into(), &[1, 2, 3, 4, 5]);
    
    // Print the Series - Polars provides nice formatted output
    // Shows the series name, data type, and values
    println!("{}", s)
}

/// Main function - entry point of the program
fn main() {
    // Call the Series demonstration function
    learn_series();
}
