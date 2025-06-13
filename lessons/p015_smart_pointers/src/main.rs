// Example of implementing a simple linked list using smart pointers in Rust
// Box<T> is a smart pointer that allows us to store data on the heap

#[derive(Debug)]  // Enable debug printing for the Node struct
struct Node {
    id: u32,      // Unique identifier for each node
    next: Option<Box<Node>>,  // Optional reference to the next node using Box
}

fn main() {
    // Example of using Box to store a simple value on the heap
    // Box::new() allocates memory on the heap and returns a pointer to it
    let x = Box::new(50);

    // Creating a linked list with three nodes (0 -> 1 -> 2)
    // Each node is stored on the heap using Box
    let nodes = Box::new(
        Node {
            id: 0,
            next: Some(
                Box::new(Node {
                    id: 1,
                    next: Some(
                        Box::new(Node {
                            id: 2,
                            next: None,  // Last node points to None
                        })
                    ),
                })
            ),
        }
    );
    
    // Print the entire linked list structure using debug formatting
    dbg!(nodes);
}
