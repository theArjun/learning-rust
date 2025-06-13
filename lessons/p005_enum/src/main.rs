// This module demonstrates enums and pattern matching in Rust
// Focus: Enum definition, variants, match expressions, and pattern matching

/// An enum representing different directions
/// Enums in Rust allow you to define a type by enumerating its possible variants
/// Each variant can be referred to by its name prefixed with the enum name
pub enum Direction {
    Up,    // Variant representing upward direction
    Down,  // Variant representing downward direction
    Left,  // Variant representing leftward direction
    Right, // Variant representing rightward direction
}

/// Main function demonstrating enum usage and pattern matching
fn main() {
    // Create an instance of the Direction enum
    // Use the :: syntax to specify which variant we want
    let dir = Direction::Up;
    
    // match expression: similar to switch statements in other languages
    // but more powerful - it must handle all possible cases (exhaustive matching)
    match dir {
        // Pattern matching: if dir is Direction::Up, execute this branch
        Direction::Up => println!("Up"),
        
        // Wildcard pattern '_' matches any remaining cases
        // This handles Down, Left, and Right variants
        _ => println!("Other direction")
    }
}
