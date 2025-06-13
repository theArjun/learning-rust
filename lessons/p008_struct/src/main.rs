// This module demonstrates structs and traits in Rust
// Focus: Struct definition, fields, traits, implementations, and object-oriented concepts

/// A simple struct representing a point in 2D space
/// Structs group related data together into a single type
pub struct Point {
    pub x: i32, // x-coordinate (public field)
    pub y: i32, // y-coordinate (public field)
}

// Usage example (commented out):
// fn main() {
//     let p = struct_::Point { x: 0, y: 0 };
//     println!("{}", p.x);
//     println!("{}", p.y);
// }

/// A struct representing a rectangle with width and height
/// Demonstrates struct with multiple fields of the same type
pub struct Rectangle {
    pub width: i32,  // Width of the rectangle (public field)
    pub height: i32, // Height of the rectangle (public field)
}

// Usage example (commented out):
// fn main() {
//     let r = struct_::Rectangle {
//         width: 10,
//         height: 10,
//     };
//     let area = r.width * r.height;
//     println!("The area of the rectangle is {} square pixels.", area);
// }

/// Traits define shared behavior that can be implemented by different types
/// Similar to interfaces in other languages
/// This trait defines a contract for types that can calculate their area
pub trait Area {
    /// Method signature for calculating area
    /// &self means this method takes an immutable reference to self
    fn area(&self) -> i32; // Like area for Rectangle
}

// Example implementation (commented out):
// impl Area for Rectangle {
//     fn area(&self) -> i32 {
//         self.width * self.height
//     }
// }

/// Implementation block for Rectangle struct
/// This implements the Area trait for Rectangle
impl Area for Rectangle {
    /// Calculate the area of the rectangle
    /// Implementation of the Area trait for Rectangle type
    fn area(&self) -> i32 {
        // Multiply width by height to get area
        self.width * self.height
    }
}

/// Main function demonstrating struct instantiation and trait usage
fn main() {
    // Create a new Rectangle instance
    // Struct instantiation using field names and values
    let r = Rectangle {
        width: 10,
        height: 10,
    };
    
    // Call the area() method from the Area trait
    // This is possible because Rectangle implements the Area trait
    println!("The area of the rectangle is {} square pixels.", r.area());
}