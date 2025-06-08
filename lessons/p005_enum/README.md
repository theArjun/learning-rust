# 🏷️ Enums

**Quick Notes:**
- **Definition:** `enum Color { Red, Green, Blue }`
- **With data:** `enum Message { Quit, Move { x: i32, y: i32 }, Write(String) }`
- **Match:** `match color { Color::Red => "red", _ => "other" }`
- **Methods:** `impl Color { fn is_red(&self) -> bool { } }`
- Built-in enums: `Option<T>`, `Result<T, E>`
- Enums can have different data types per variant

**Run:** `cargo run` 