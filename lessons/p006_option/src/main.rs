pub fn find_one(id: u32) -> Option<String> {
    if id == 1 {
        Some(String::from("1 found."))
    } else {
        None
    }
}

// Usage:
fn main() {
    let num = 2;
    let result = find_one(num);
    match result {
        Some(value) => println!("{}", value),
        None => println!("None")
    }
}

