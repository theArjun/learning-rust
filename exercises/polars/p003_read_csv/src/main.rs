// This module demonstrates reading CSV files using Polars
// Focus: File I/O, CSV parsing, and error handling with Result types

// Import Polars prelude for DataFrame and CSV reading functionality
use polars::prelude::*;
// Import standard library File type for file operations
use std::fs::File;

/// Main function demonstrating CSV file reading with Polars
/// Returns PolarsResult<()> to handle potential errors gracefully
fn main() -> PolarsResult<()> {
    // === FILE OPENING ===
    
    // Specify the path to your CSV file
    // In this example, it expects "employees.csv" to be in the current directory
    let file = File::open("employees.csv").unwrap();
    
    // Note: Using unwrap() here for simplicity, but in production code
    // you'd want to handle file opening errors more gracefully
    
    // === CSV READING ===
    
    // Read CSV file into a DataFrame using CsvReader
    // CsvReader provides many configuration options for parsing
    let df = CsvReader::new(file)        // Create reader from the file handle
        .finish()                        // Execute the reading operation
        .unwrap();                       // Extract DataFrame or panic on error
    
    // Alternative: Using ? operator for error propagation:
    // let df = CsvReader::new(file).finish()?;
    
    // === DISPLAY RESULTS ===
    
    // Print the DataFrame - shows structure, column names, types, and data
    // Polars automatically formats the output in a readable table format
    println!("{}", df);
    
    // Return Ok(()) to indicate successful completion
    // This satisfies the PolarsResult<()> return type
    Ok(())
}