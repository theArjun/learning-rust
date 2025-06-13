use std::collections::{HashMap, HashSet};

fn main() {
    // HashMap
    let person_1: &str = "Arjun";
    let person_2: &str = "Liza";

    let mut results_hm: HashMap<&str, u32> = HashMap::new();

    results_hm.insert(person_1, 25);
    results_hm.insert(person_2, 21);

    let liza_age: Option<&u32> = results_hm.get(person_2);
    println!("Liza Age: {}", liza_age.unwrap());

    // Check by key in hashset
    if results_hm.contains_key("Arjun"){
        println!("Arjun exists in the HashSet.")
    }

    // HashSet
    let mut names_hs: HashSet<&str> = HashSet::new();
    names_hs.insert("Arjun");
    names_hs.insert("Liza");

    // Test String exists in HashSet
    if names_hs.contains("Arjun"){
        println!("Arjun exists in names hashset.")
    }
}
