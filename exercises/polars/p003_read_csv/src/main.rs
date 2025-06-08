use polars::prelude::*;
use std::fs::File;

fn main() -> PolarsResult<()> {
    // Specify the path to your CSV file
    let file = File::open("employees.csv").unwrap();
    
    // Read CSV file into a DataFrame
    let df = CsvReader::new(file)
        .finish()
        .unwrap();
    
    // Print the DataFrame
    println!("{}", df);
    
    Ok(())
}