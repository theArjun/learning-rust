// This module demonstrates date range generation and weekday calculation
// Focus: Date manipulation, module usage, and iterator patterns with Chrono

// Import the get_date_ranges function from our local date_util module
use crate::date_util::get_date_ranges;
// Import Chrono types for date operations
// Datelike trait provides weekday() method
// NaiveDate represents dates without timezone information
use chrono::{Datelike, NaiveDate};

// Declare the date_util module (corresponds to date_util.rs file)
mod date_util;

/// Main function demonstrating date range generation and weekday extraction
fn main() {
    // === DATE CREATION ===
    
    // Create start date: January 1, 2023
    // from_ymd_opt returns Option<NaiveDate> to handle invalid dates safely
    // expect() extracts the value or panics with a custom message if None
    let start_date = NaiveDate::from_ymd_opt(2023, 1, 1).expect("Invalid start date");
    
    // Create end date: February 28, 2023
    // This creates a date range covering almost 2 months
    let end_date = NaiveDate::from_ymd_opt(2023, 2, 28).expect("Invalid end date");
    
    // === DATE RANGE GENERATION ===
    
    // Generate a vector of all dates between start_date and end_date (inclusive)
    // Calls our custom function from the date_util module
    let date_ranges = get_date_ranges(start_date, end_date);

    // === WEEKDAY EXTRACTION ===
    
    // Iterate through each date in the range
    // For each date, print its weekday (Monday, Tuesday, etc.)
    for date in date_ranges {
        // weekday() method from Datelike trait returns the day of the week
        // This will print: Monday, Tuesday, Wednesday, etc.
        println!("{} ", date.weekday())
    }
}