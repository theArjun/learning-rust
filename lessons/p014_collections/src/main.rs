// This module demonstrates Rust's standard collection types
// Focus: HashMap (key-value pairs) and HashSet (unique values)

// Import HashMap and HashSet from the standard collections module
use std::collections::{HashMap, HashSet};

/// Main function demonstrating HashMap and HashSet usage
fn main() {
    // === HASHMAP EXAMPLE ===
    // HashMap stores key-value pairs and provides O(1) average lookup time
    
    // Define string literals to use as keys
    let person_1: &str = "Arjun";
    let person_2: &str = "Liza";

    // Create a new HashMap that maps string keys to u32 values
    // HashMap<&str, u32> means keys are string slices, values are 32-bit unsigned integers
    let mut results_hm: HashMap<&str, u32> = HashMap::new();

    // Insert key-value pairs into the HashMap
    // Each person's name is associated with their age
    results_hm.insert(person_1, 25);  // "Arjun" -> 25
    results_hm.insert(person_2, 21);  // "Liza" -> 21

    // Retrieve a value by key - returns Option<&u32>
    // get() returns Some(&value) if key exists, None if it doesn't
    let liza_age: Option<&u32> = results_hm.get(person_2);
    
    // Extract the value using unwrap() - will panic if None
    // In production code, you'd want to handle the None case safely
    println!("Liza Age: {}", liza_age.unwrap());

    // Check if a key exists in the HashMap without retrieving the value
    // contains_key() returns a boolean indicating presence of the key
    if results_hm.contains_key("Arjun"){
        // Note: The comment says "HashSet" but this is actually a HashMap check
        println!("Arjun exists in the HashMap.")
    }

    // === HASHSET EXAMPLE ===
    // HashSet stores unique values and provides O(1) average lookup/insert time
    
    // Create a new HashSet that stores string slice values
    // HashSet ensures all values are unique (no duplicates)
    let mut names_hs: HashSet<&str> = HashSet::new();
    
    // Insert values into the HashSet
    // If we try to insert the same value twice, only one copy is stored
    names_hs.insert("Arjun");
    names_hs.insert("Liza");

    // Check if a value exists in the HashSet
    // contains() returns true if the value is present, false otherwise
    if names_hs.contains("Arjun"){
        println!("Arjun exists in names hashset.")
    }
}
