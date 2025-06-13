// This module demonstrates asynchronous programming in Rust
// Focus: async/await syntax, HTTP requests, and async runtimes

/// Asynchronous function to make HTTP requests and parse JSON responses
/// 
/// The 'async' keyword makes this function asynchronous, meaning it can be paused
/// and resumed without blocking the thread
/// 
/// Parameters:
/// - url: string slice containing the URL to request
/// 
/// Returns: Result containing either a JSON value or a reqwest error
async fn make_request(url: &str) -> Result<serde_json::Value, reqwest::Error> {
    // Make HTTP GET request asynchronously
    // The 'await' keyword pauses execution until the operation completes
    // The '?' operator propagates errors up to the caller
    let response: serde_json::Value = reqwest::get(url)
        .await?                           // Wait for HTTP request to complete
        .json::<serde_json::Value>()      // Parse response as JSON
        .await?;                          // Wait for JSON parsing to complete
    
    // Return the successful result wrapped in Ok
    Ok(response)
}

/// Asynchronous function to fetch and display NEPSE stock index data
/// Demonstrates error handling with async functions
async fn get_nepse_index() {
    // API endpoint for Nepal Stock Exchange index data
    let api_url: &str = "https://data.nepse.bot/todays-index/NEPSE";
    
    // Call our async function and wait for the result
    // The variable name has a typo (should be "response")
    let my_repsonse: Result<serde_json::Value, reqwest::Error> = make_request(api_url).await;
    
    // Handle the Result using pattern matching
    match my_repsonse {
        // If the request succeeded, print the response data
        Ok(r) => {
            // dbg! macro prints debug information about the value
            dbg!(r);
        }
        // If the request failed, panic (not recommended for production)
        Err(_) => {
            // In production, you'd want to handle errors more gracefully
            panic!("Failed to make request");
        }
    }
}

/// Main function using Tokio async runtime
/// 
/// The #[tokio::main] attribute transforms this function into an async main
/// Tokio provides the async runtime that executes async functions
#[tokio::main]
async fn main() {
    // Call our async function and wait for it to complete
    // Without 'await', the function would return immediately without executing
    get_nepse_index().await;
}
