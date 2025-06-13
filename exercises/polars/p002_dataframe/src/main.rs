// This module demonstrates DataFrame creation and usage with Polars
// Focus: DataFrame construction with mixed data types, date handling, and display

// Import Chrono prelude for date/time handling
use chrono::prelude::*;
// Import Polars prelude for DataFrame operations
use polars::prelude::*;

/// Function to create and display a sample DataFrame
/// Demonstrates creating a DataFrame with multiple column types:
/// - String data (names)
/// - Date data (birthdates)  
/// - Numeric data (weight, height)
fn sample_df() {
    // Create a DataFrame using the df! macro
    // The df! macro provides a convenient way to create DataFrames
    let df: DataFrame =
        df!( // If we want to mutate the DF, use mut.
            // String column: person names
            "name" => ["Alice Archer", "Ben Brown", "Chloe Cooper", "Daniel Donovan"],
            
            // Date column: birthdates using NaiveDate
            // NaiveDate represents dates without timezone information
            // from_ymd_opt returns Option<NaiveDate> - unwrap() extracts the value
            "birthdate" => [
                NaiveDate::from_ymd_opt(1997, 1, 10).unwrap(),  // January 10, 1997
                NaiveDate::from_ymd_opt(1985, 2, 15).unwrap(),  // February 15, 1985
                NaiveDate::from_ymd_opt(1983, 3, 22).unwrap(),  // March 22, 1983
                NaiveDate::from_ymd_opt(1981, 4, 30).unwrap(),  // April 30, 1981
            ],
            
            // Numeric columns: floating-point data
            "weight" => [57.9, 72.5, 53.6, 83.1],  // Weight in kilograms
            "height" => [1.56, 1.77, 1.65, 1.75],  // Height in meters
        )
        .unwrap(); // Unwrap the Result<DataFrame, PolarsError>
    
    // Print the DataFrame - Polars provides nicely formatted tabular output
    // Shows column names, data types, and all rows with proper alignment
    println!("{}", df);
}

/// Main function - entry point of the program
fn main() {
    // Call the DataFrame demonstration function
    sample_df();
}
