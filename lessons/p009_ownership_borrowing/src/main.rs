// What is Ownership?
// Rust’s ownership system manages memory. Each value has a single owner, and when the owner goes out of scope, the value is dropped automatically.

// Borrowing and References
// You can borrow references to values using & (immutable) or &mut (mutable):

// These ampersands represent references, and they allow you to refer to some value
// without taking ownership of it [i.e. borrowing].

// The opposite of referencing by using & is dereferencing,
// which is accomplished with the dereference operator, *.
pub fn calculate_length(s: &String) -> usize {
    s.len() // Returns
}

// Usage:
//     let s1= String::from("hello");
//     let length = ownership_borrowing::calculate_length(&s1);
//     println!("The length of '{}' is {}.", s1, length);

pub fn check_referencing() -> bool {
    let x = 5;
    let y = &x;

    assert_eq!(x, 5);
    assert_eq!(*y, 5);

    true
}

// Function without taking ownership
pub fn calculate_substring_length(s: &str) -> usize {
    s.len()
}

// Usage:
fn main() {
    let s = String::from("hello");
    let slice = &s[0..3]; // hel
    let length = calculate_substring_length(&slice);
    println!("The length of {} is {}", slice, length);
}