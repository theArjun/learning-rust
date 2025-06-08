# 🏗️ Structs

**Quick Notes:**
- **Definition:** `struct User { name: String, age: u32 }`
- **Create:** `let user = User { name: "Alice".to_string(), age: 30 };`
- **Access:** `user.name`, `user.age`
- **Update:** `User { age: 31, ..user }` (struct update syntax)
- **Tuple struct:** `struct Point(f64, f64, f64);`
- **Unit struct:** `struct AlwaysEqual;`
- **Methods:** `impl User { fn greet(&self) { } }`

**Run:** `cargo run` 