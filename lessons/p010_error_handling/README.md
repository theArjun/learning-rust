# ⚠️ Error Handling

**Quick Notes:**
- **Result<T, E>:** `Ok(value)` or `Err(error)`
- **Match:** `match result { Ok(v) => v, Err(e) => panic!() }`
- **Shortcuts:** `.unwrap()`, `.expect("message")`, `.unwrap_or(default)`
- **Propagate:** `?` operator - `let x = might_fail()?;`
- **Custom errors:** Create your own error types
- **Panic:** `panic!("message")` for unrecoverable errors
- No exceptions - explicit error handling!

**Run:** `cargo run` 