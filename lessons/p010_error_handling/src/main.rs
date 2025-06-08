// The Result Type
// Used for functions that might fail:

pub fn divide(number: i32, divisor: i32) -> Result<i32, String> {
    if divisor == 0 {
        return Err("Cannot divide by zero".to_string());
    }
    Ok(number / divisor)
}

// Usage:
fn main() {
    let number = 40;
    let divisor = 0;
//
//      Way 1: Use unwrap to get the value or panic if it fails; can panic.
//      unwrap() will panic (crash your program) if the result is an Err.
//
     let result = divide(number, divisor);
//      let value = result.unwrap();
//      println!("The result is {}", value);
//
    // Way 2: Use match
    match result {
        Ok(value) => println!("{}", value),
        Err(e) => println!("{}", e),
    }
//
//     // Way 3: Using if let
//     if let Ok(quotient) = quotient{
//         println!("{}", quotient);
//     } else {
//         println!("{}", number); // Can be printed anything, But I chose to print number.
//     }
}

// Summary
// match: Best for handling both success and error explicitly.
// if let: Great for a concise check when you mainly care about the successful case.
// unwrap() / expect(): Handy for prototyping or when you're sure an error won't occur, but be cautious!
