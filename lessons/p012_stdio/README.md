# 🖥️ Standard I/O

**Quick Notes:**

- **Input:** `use std::io;` then `io::stdin().read_line(&mut string)`
- **Output:** `println!("Hello {}", value);` or `print!("No newline");`
- **Parse input:** `input.trim().parse::<i32>().unwrap()`
- **Handle errors:** `.expect("message")` or `.unwrap_or(default)`
- **Multiple inputs:** Read each line separately
- **Trim whitespace:** Always `.trim()` input before parsing
- Interactive programs need mutable `String::new()`

**Run:** `cargo run`
