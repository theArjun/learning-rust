// This module demonstrates smart pointers in Rust, specifically Box<T>
// Focus: Heap allocation, recursive data structures, and smart pointer ownership

// Example of implementing a simple linked list using smart pointers in Rust
// Box<T> is a smart pointer that allows us to store data on the heap
// Box provides ownership of the heap-allocated data and cleans up when dropped

/// A node in a linked list implemented using Box smart pointers
/// 
/// Box<T> is necessary here because Rust needs to know the size of types at compile time
/// Without Box, a recursive type like Node would have infinite size
#[derive(Debug)]  // Enable debug printing for the Node struct
struct Node {
    id: u32,      // Unique identifier for each node (stored on stack within the Box)
    next: Option<Box<Node>>,  // Optional reference to the next node using Box
                              // Option handles the case where there's no next node
                              // Box ensures the Node is heap-allocated
}

/// Main function demonstrating Box<T> smart pointer usage
fn main() {
    // === SIMPLE BOX EXAMPLE ===
    
    // Example of using Box to store a simple value on the heap
    // Box::new() allocates memory on the heap and returns a pointer to it
    // The value 50 is stored on the heap instead of the stack
    let x = Box::new(50);
    println!("Value stored in Box: {}", x); // Box automatically dereferences

    // === LINKED LIST EXAMPLE ===
    
    // Creating a linked list with three nodes (0 -> 1 -> 2 -> None)
    // Each node is stored on the heap using Box
    // This demonstrates how Box enables recursive data structures
    let nodes = Box::new(
        Node {
            id: 0,  // First node with id 0
            next: Some(
                Box::new(Node {
                    id: 1,  // Second node with id 1
                    next: Some(
                        Box::new(Node {
                            id: 2,  // Third node with id 2
                            next: None,  // Last node points to None (end of list)
                        })
                    ),
                })
            ),
        }
    );
    
    // Print the entire linked list structure using debug formatting
    // The Debug trait allows us to see the nested structure
    dbg!(nodes);
    
    // Note: When 'nodes' goes out of scope, Box automatically deallocates
    // all the heap memory for the entire linked list recursively
}
