// Utility module for date range operations
// Focus: Date arithmetic, vector operations, and FFI-compatible functions

// Import Chrono types for date operations
// Duration allows us to represent time spans (days, hours, etc.)
// NaiveDate represents calendar dates without timezone information
use chrono::{Duration, NaiveDate};

/// Generate a vector of consecutive dates between start and end dates (inclusive)
/// 
/// This function is marked with FFI attributes, making it callable from C
/// though in this case it's primarily used within the Rust codebase
/// 
/// Attributes:
/// - #[no_mangle]: Preserves function name for potential external access
/// - pub extern "C": Makes function public with C calling conventions
/// 
/// Parameters:
/// - start_date: the first date in the range (inclusive)
/// - end_date: the last date in the range (inclusive)
/// 
/// Returns: Vec<NaiveDate> containing all dates from start to end
/// 
/// Example:
/// ```
/// let start = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
/// let end = NaiveDate::from_ymd_opt(2023, 1, 3).unwrap();
/// let dates = get_date_ranges(start, end);
/// // Returns: [2023-01-01, 2023-01-02, 2023-01-03]
/// ```
#[no_mangle]
pub extern "C" fn get_date_ranges(start_date: NaiveDate, end_date: NaiveDate) -> Vec<NaiveDate> {
    // Create an empty vector to store the date sequence
    let mut date_ranges = Vec::new();
    
    // Initialize current date to the start date
    // This will be incremented by one day in each iteration
    let mut current_date = start_date;

    // Continue while current date hasn't passed the end date
    // This creates an inclusive range [start_date, end_date]
    while current_date <= end_date {
        // Add the current date to our result vector
        date_ranges.push(current_date);
        
        // Move to the next day by adding a 1-day duration
        // Duration::days(1) creates a span of exactly one day
        current_date = current_date + Duration::days(1);
    }
    
    // Return the complete vector of dates
    date_ranges
}