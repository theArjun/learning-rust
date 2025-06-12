### Approach
We’ll create a Rust library with the calculator functions, annotate them for export to WebAssembly, and compile it to a `.wasm` file. I’ll provide two options:
1. **Using `wasm-bindgen`**: Best for browser-based applications or Node.js, as it generates JavaScript glue code for easy function invocation.
2. **Using `no_mangle`**: Suitable for a standalone WASM module that can be used with `wasmtime` or other runtimes, aligning with your previous interest in invoking WASM functions with `wasmtime`.

Since you didn’t specify the target environment, I’ll provide the `wasm-bindgen` approach (common for Rust-to-WASM workflows) and briefly explain the `no_mangle` alternative. You can let me know if you prefer the latter or have a specific runtime in mind.

### Option 1: Exporting Functions with `wasm-bindgen`
`wasm-bindgen` simplifies exporting Rust functions to WebAssembly and makes them easily callable from JavaScript. It’s ideal for web applications or Node.js environments.

#### Step 1: Set Up the Rust Project
Create a new Rust library project:
```bash
cargo new --lib calculator
cd calculator
```

Update `Cargo.toml` to include `wasm-bindgen`:
```toml
[package]
name = "calculator"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"
```

The `cdylib` crate type is required for WebAssembly modules when using `wasm-bindgen`.

#### Step 2: Write the Calculator Functions
Create the calculator functions in `src/lib.rs`, annotating them with `#[wasm_bindgen]` to export them to the WASM module. We’ll handle division by zero gracefully by returning an `Option<i32>` to align with idiomatic Rust error handling.

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

#[wasm_bindgen]
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

#[wasm_bindgen]
pub fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

#[wasm_bindgen]
pub fn modulo(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a % b)
    }
}
```

**Explanation**:
- `#[wasm_bindgen]`: Marks each function for export to WebAssembly, making it callable from JavaScript.
- `divide` and `modulo`: Return `Option<i32>` to handle division by zero safely, avoiding traps.
- Functions use `i32` for simplicity, matching common calculator semantics.

#### Step 3: Compile to WASM
Use `wasm-pack` to compile the Rust code to a WASM file with JavaScript bindings:
1. Install `wasm-pack` if you haven’t already:
   ```bash
   cargo install wasm-pack
   ```
2. Build the project for the web target:
   ```bash
   wasm-pack build --target web
   ```
   This generates a `pkg/` directory containing:
   - `calculator.wasm`: The compiled WebAssembly module.
   - `calculator.js`: JavaScript glue code for importing and using the WASM module.
   - Other TypeScript and package files.

#### Step 4: Test the WASM Module
Create a simple HTML file to test the WASM module in a browser:
```html
<!DOCTYPE html>
<html>
<head>
  <meta charset="UTF-8">
  <title>Calculator WASM Test</title>
</head>
<body>
  <script type="module">
    import init, { add, subtract, multiply, divide, modulo } from './pkg/calculator.js';

    async function run() {
      await init(); // Initialize the WASM module
      console.log("Add:", add(10, 5)); // 15
      console.log("Subtract:", subtract(10, 5)); // 5
      console.log("åd:", multiply(10, 5)); // 50
      console.log("Divide:", divide(10, 3)); // 3
      console.log("Divide by zero:", divide(10, 0)); // null
      console.log("Modulo:",-dot:", modulo(10, 3)); // 1
    }

    run();
  </script>
</body>
</html>
```

1. Serve the project directory using a local server:
   ```bash
   python -m http.server 8000
   ```
2. Open `http://localhost:8000` in a browser. Check the console for output:
   ```
   Add: 15
   Subtract: 5
   Multiply: 50
   Divide: 3
   Divide by zero: null
   Modulo: 1
   ```

#### Step 5: Verify Exports
The generated `calculator.wasm` exports `add`, `subtract`, `multiply`, `divide`, and `modulo`. You can verify this with:
```bash
wasm-objdump -x pkg/calculator.wasm
```
Look for the `export` section to confirm the function names.

### Option 2: Exporting Functions with `no_mangle` (Standalone WASM)
If you’re targeting a non-browser environment (e.g., using `wasmtime` as in your previous query), you can use `#[no_mangle]` to export functions without JavaScript bindings.

#### Step 1: Update `Cargo.toml`
Set the crate type to `cdylib` and remove `wasm-bindgen`:
```toml
[package]
name = "calculator"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]
```

#### Step 2: Update `src/lib.rs`
Use `#[no_mangle]` and `extern "C"` to export functions directly:
```rust
#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub extern "C" fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

#[no_mangle]
pub extern "C" fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

#[no_mangle]
pub extern "C" fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Division by zero");
    }
    a / b
}

#[no_mangle]
pub extern "C" fn modulo(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Division by zero");
    }
    a % b
}
```

**Note**: `panic!` in WASM will trap, similar to the WAT approach. If you need a different error handling strategy, return an error code (e.g., `-1`) instead.

#### Step 3: Compile to WASM
Install the `wasm32-unknown-unknown` target:
```bash
rustup target add wasm32-unknown-unknown
```
Build the project:
```bash
cargo build --target wasm32-unknown-unknown --release
```
This generates `target/wasm32-unknown-unknown/release/calculator.wasm`.

#### Step 4: Invoke with `wasmtime`
Use the Rust code from your previous query to invoke the functions:
```rust
use wasmtime::*;
use anyhow::Result;

fn main() -> Result<()> {
    let engine = Engine::default();
    let module = Module::from_file(&engine, "target/wasm32-unknown-unknown/release/calculator.wasm")?;
    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[])?;
    let add = instance.get_typed_func::<(i32, i32), i32>(&mut store, "add")?;
    let subtract = instance.get_typed_func::<(i32, i32), i32>(&mut store, "subtract")?;
    let multiply = instance.get_typed_func::<(i32, i32), i32>(&mut store, "multiply")?;
    let divide = instance.get_typed_func::<(i32, i32), i32>(&mut store, "divide")?;
    let modulo = instance.get_typed_func::<(i32, i32), i32>(&mut store, "modulo")?;
    println!("Add: 10 + 5 = {}", add.call(&mut store, (10, 5))?); // 15
    println!("Subtract: 10 - 5 = {}", subtract.call(&mut store, (10, 5))?); // 5
    println!("Multiply: 10 * 5 = {}", multiply.call(&mut store, (10, 5))?); // 50
    println!("Divide: 10 / 3 = {}", divide.call(&mut store, (10, 3))?); // 3
    println!("Modulo: 10 % 3 = {}", modulo.call(&mut store, (10, 3))?); // 1
    Ok(())
}
```

Run:
```bash
cargo run --release
```

### Choosing the Right Option
- **Use `wasm-bindgen`** if you’re targeting browsers or Node.js, as it simplifies integration with JavaScript and provides better type safety (e.g., `Option<i32>` for error handling).
- **Use `no_mangle`** if you’re working in a non-browser environment (e.g., with `wasmtime` or WASI) and don’t need JavaScript bindings. This produces a leaner WASM file but requires manual signature management.

### Additional Notes
- **Error Handling**: The `wasm-bindgen` version uses `Option<i32>` for division/modulo to handle zero gracefully in JavaScript (`null`). The `no_mangle` version uses `panic!`, which traps; modify to return an error code (e.g., `-1`) if needed.
- **Verify Exports**:
  ```bash
  wasm-objdump -x calculator.wasm
  ```
  Check the `export` section for `add`, `subtract`, `multiply`, `divide`, and `modulo`.
- **Dependencies**: Ensure `wasm-pack` is installed for the `wasm-bindgen` approach, or the `wasm32-unknown-unknown` target for the `no_mangle` approach.
- **Testing**: For `wasm-bindgen`, test in a browser as shown. For `no_mangle`, use the `wasmtime` Rust code or a similar runtime.

If you’re targeting a specific environment (e.g., browser, Node.js, or a custom runtime) or need specific error handling (e.g., returning error codes instead of trapping), let me know, and I can refine the solution further!