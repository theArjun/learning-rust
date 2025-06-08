# Rust Lifetimes - Sticky Notes 📌

## 🎯 **What are Lifetimes?**
- **Think of lifetimes as "rental agreements" for memory**
- They ensure references don't outlive the data they point to
- Prevent **dangling pointers** (pointing to freed memory)

## 🔑 **Key Rules to Remember**

### **Rule #1: The Golden Rule**
> **"A reference cannot outlive the data it references"**

### **Rule #2: Function Parameters**
```rust
// When multiple refs go in, Rust needs to know which lifetime comes out
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { ... }
```

### **Rule #3: Struct Fields**
```rust
// If struct holds a reference, it needs a lifetime
struct ImportantExcerpt<'a> {
    part: &'a str,  // This ref must live as long as the struct
}
```

## 🧠 **Mental Models**

### **The Landlord Analogy**
- **Data owner** = Landlord 🏠
- **Reference** = Tenant 👤
- **Lifetime** = Lease duration 📅
- Rule: Tenant can't stay after landlord sells the house!

### **The Library Book Model**
- You can **borrow** a book (reference)
- But you can't keep it longer than the library is open (data lifetime)
- Multiple people can read the same book (multiple immutable refs)
- Only one person can write in it at a time (one mutable ref)

## ⚡ **Quick Syntax Cheat Sheet**

```rust
// Basic lifetime annotation
&'a T          // Reference with lifetime 'a

// Common patterns
<'a>           // Lifetime parameter
where T: 'a    // T must live at least as long as 'a
'static        // Lives for entire program duration
```

## 🚨 **Common Gotchas & Fixes**

### **Problem: Returning References**
```rust
// ❌ This won't compile
fn bad_function() -> &str {
    let s = String::from("hello");
    &s  // s dies here, but we're returning a reference to it!
}

// ✅ Fix: Return owned data
fn good_function() -> String {
    String::from("hello")  // Caller owns this
}
```

### **Problem: Multiple References**
```rust
// ❌ Ambiguous lifetimes
fn longest(x: &str, y: &str) -> &str { ... }

// ✅ Explicit lifetimes
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { ... }
```

## 🎪 **Lifetime Elision (The Magic Tricks)**

Rust can **auto-infer** lifetimes in these cases:
1. **Single input reference** → output gets same lifetime
2. **Method with &self** → output gets &self's lifetime
3. **Simple cases** → Rust figures it out

```rust
// These are equivalent:
fn first_word(s: &str) -> &str { ... }
fn first_word<'a>(s: &'a str) -> &'a str { ... }
```

## 💡 **Pro Tips**

1. **Start without lifetimes** - add them only when compiler complains
2. **'static is forever** - use sparingly (string literals, constants)
3. **Clone when stuck** - sometimes ownership is simpler than borrowing
4. **Use lifetime elision** - let Rust infer when possible

## 🔍 **Debug Mantra**
When lifetime errors occur, ask:
1. **"Who owns this data?"**
2. **"How long does it live?"**
3. **"Am I trying to use it after it's gone?"**

---
*Remember: Lifetimes are about **safety**, not performance. Rust prevents use-after-free bugs at compile time!* 🦀
