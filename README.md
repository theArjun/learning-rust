# Rust Project Creation Guide

## Creating New Rust Projects

### Binary Project (Executable)

```bash
cargo new project_name
cd project_name
```

### Library Project

```bash
cargo new --lib library_name
cd library_name
```

### Binary Project in Current Directory

```bash
cargo init
```

### Library Project in Current Directory

```bash
cargo init --lib
```

## Project Structure

### Binary Project

```
project_name/
├── Cargo.toml
└── src/
    └── main.rs
```

### Library Project

```
library_name/
├── Cargo.toml
└── src/
    └── lib.rs
```

## Working with Modules

### Creating Modules in Same File

```rust
mod my_module {
    pub fn my_function() {
        println!("Hello from module");
    }
}

fn main() {
    my_module::my_function();
}
```

### Creating Modules in Separate Files

#### Single File Module

Create `src/my_module.rs`:

```rust
pub fn my_function() {
    println!("Hello from module");
}
```

In `src/main.rs` or `src/lib.rs`:

```rust
mod my_module;

fn main() {
    my_module::my_function();
}
```

#### Directory Module

Create `src/my_module/mod.rs`:

```rust
pub fn my_function() {
    println!("Hello from module");
}
```

Or create `src/my_module.rs` and `src/my_module/submodule.rs`

### Module Visibility

- `pub` - public, can be used from outside
- No `pub` - private, only within module

## Adding Dependencies

Edit `Cargo.toml`:

```toml
[dependencies]
serde = "1.0"
tokio = { version = "1.0", features = ["full"] }
```

## Common Commands

### Build and Run

```bash
cargo build          # Build project
cargo run            # Build and run
cargo check          # Check compilation without building
cargo test           # Run tests
cargo doc            # Generate documentation
```

### Release Build

```bash
cargo build --release
cargo run --release
```

### Adding Examples

Create `examples/example_name.rs`:

```bash
cargo run --example example_name
```

### Workspaces

Create `Cargo.toml` in root:

```toml
[workspace]
members = [
    "project1",
    "project2",
    "library1"
]
```

## Library Crate Usage

### In lib.rs

```rust
pub mod my_module;

pub fn public_function() {
    println!("This is public");
}

fn private_function() {
    println!("This is private");
}
```

### Using External Crates

```rust
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
```

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

### Integration Tests

Create `tests/integration_test.rs`:

```rust
use library_name;

#[test]
fn test_something() {
    // test code
}
```

Run tests:

```bash
cargo test
cargo test test_name
cargo test --lib
cargo test --bin
```
