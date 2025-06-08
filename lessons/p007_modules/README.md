# 📦 Modules

**Quick Notes:**

- **Define:** `mod module_name { }` or `mod module_name;` (separate file)
- **Public:** `pub fn function_name()`, `pub struct Name`
- **Use:** `use crate::module::function;`, `use std::collections::HashMap;`
- **Path:** `crate::` (current crate), `super::` (parent), `self::`
- **Re-export:** `pub use module::Item;`
- Files: `lib.rs`/`main.rs` are crate roots
- Organize code into logical groups

**Run:** `cargo run` 