# 🔐 Ownership & Borrowing

**Quick Notes:**
- **Ownership Rules:** Each value has one owner, owner cleans up
- **Move:** `let y = x;` (x no longer valid for non-Copy types)
- **Borrow:** `&x` (immutable reference), `&mut x` (mutable reference)
- **Rules:** One mutable OR many immutable references at a time
- **Scope:** References must be valid as long as they're used
- **Copy vs Move:** `i32` copies, `String` moves
- Memory safety without garbage collector!

**Run:** `cargo run` 