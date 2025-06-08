# 🎯 Option

**Quick Notes:**
- **Option<T>:** `Some(value)` or `None` (no null pointers!)
- **Match:** `match opt { Some(x) => x, None => 0 }`
- **Methods:** `.unwrap()`, `.unwrap_or(default)`, `.expect("msg")`
- **Safe access:** `.is_some()`, `.is_none()`
- **Transform:** `.map(|x| x + 1)`, `.filter(|&x| x > 0)`
- **If let:** `if let Some(x) = option { }`
- Rust's way to handle "maybe" values safely

**Run:** `cargo run` 