# Using WebAssembly (WASM) with Rust

This project demonstrates how to load and execute WebAssembly modules using Rust and the `wasmtime` runtime. This guide covers common issues you might encounter and their solutions.

## Overview

WebAssembly (WASM) is a binary instruction format that allows you to run code compiled from various languages (like Rust, C, C++, AssemblyScript) in a sandboxed environment. This project shows how to:

- Load a WASM module from a `.wasm` file
- Handle imports required by the WASM module
- Call exported functions from the WASM module
- Handle different function signatures and return types

## Dependencies

Add these dependencies to your `Cargo.toml`:

```toml
[dependencies]
anyhow = "1.0.98"
wasmtime = "33.0.0"
wasmtime-wasi = "33.0.0"  # Optional: for WASI support
```

## Basic Usage

```rust
use wasmtime::*;
use anyhow::Result;

fn main() -> Result<()> {
    // Create the WebAssembly engine
    let engine = Engine::default();

    // Load the WASM file
    let module = Module::from_file(&engine, "your_module.wasm")?;

    // Create a store (runtime state)
    let mut store = Store::new(&engine, ());

    // Create a linker for handling imports
    let mut linker = Linker::new(&engine);

    // Handle any imports the module requires (see below)
    
    // Instantiate the module
    let instance = linker.instantiate(&mut store, &module)?;

    // Get and call exported functions
    let my_func = instance.get_typed_func::<(i32, i32), i32>(&mut store, "my_function")?;
    let result = my_func.call(&mut store, (10, 5))?;

    Ok(())
}
```

## Common Issues and Solutions

### 1. Import Resolution Errors

**Problem**: `Error: unknown import: 'module::function' has not been defined`

**Cause**: Your WASM module requires imports that aren't being provided by your Rust code.

**Solutions**:

#### For wasm-bindgen Generated Modules
If your WASM was created using `wasm-bindgen`, it will require specific imports:

```rust
// Add the required wasm-bindgen import
linker.func_wrap("wbg", "__wbindgen_init_externref_table", || {})?;
```

#### For Custom Imports
Use `wasm-tools` to inspect what imports your module needs:

```bash
# Install wasm-tools
cargo install wasm-tools

# Inspect your WASM module
wasm-tools print your_module.wasm | head -30
```

Then provide the required imports:

```rust
// Example: providing a simple function import
linker.func_wrap("env", "my_import_function", |x: i32| -> i32 {
    x * 2
})?;
```

### 2. Function Type Mismatches

**Problem**: `Error: type mismatch: expected i32, found f64`

**Cause**: The function signature you're using doesn't match the actual WASM function signature.

**Solution**: Check the actual function signatures in your WASM module:

```bash
# Look for function type definitions
wasm-tools print your_module.wasm | grep -E "(type.*func|export.*func)"
```

Then use the correct types:

```rust
// If the function returns f64 instead of i32:
let divide = instance.get_typed_func::<(i32, i32), f64>(&mut store, "divide")?;

// If the function takes f32 parameters:
let float_func = instance.get_typed_func::<(f32, f32), f32>(&mut store, "my_func")?;
```

### 3. File Loading Issues

**Problem**: `Error: No such file or directory` or `Error: failed to read wasm file`

**Solutions**:
- Ensure the WASM file exists in the correct location
- Use absolute paths if needed
- Check file permissions

```rust
// Using absolute path
let module = Module::from_file(&engine, "/absolute/path/to/module.wasm")?;

// Or check if file exists first
use std::path::Path;
if !Path::new("module.wasm").exists() {
    panic!("WASM file not found!");
}
```

## Debugging WASM Modules

### Inspecting WASM Structure

```bash
# View the entire module structure
wasm-tools print calculator.wasm

# View just the imports and exports
wasm-tools print calculator.wasm | grep -E "(import|export)"

# View function type signatures
wasm-tools print calculator.wasm | grep -A 10 -B 5 "type.*func"
```

### Understanding Function Types

WASM function types follow this pattern:
- `(type (;N;) (func (param type1 type2) (result return_type)))`

Common types:
- `i32` - 32-bit integer
- `i64` - 64-bit integer  
- `f32` - 32-bit float
- `f64` - 64-bit float

## Key Findings from This Project

### 1. wasm-bindgen Modules Need Specific Imports

When working with WASM modules compiled with `wasm-bindgen`:
- They require imports under the `"wbg"` namespace
- Must provide `__wbindgen_init_externref_table` function
- May need other wasm-bindgen specific functions

### 2. Function Return Types Matter

Different operations may return different types:
- Mathematical operations like `add`, `subtract`, `multiply` often return `i32`
- Division operations (`divide`) commonly return `f64` for precision
- Modulo operations may also return `f64` in some implementations

### 3. Always Inspect Your WASM Module

Before writing Rust code to load a WASM module:
1. Use `wasm-tools print` to understand the structure
2. Check what imports are required
3. Verify the exact function signatures
4. Note any special requirements (like wasm-bindgen)

## Best Practices

1. **Error Handling**: Always use proper error handling with `Result<()>`
2. **Type Safety**: Use `get_typed_func` to ensure type safety
3. **Resource Management**: The `Store` manages runtime state - reuse it for multiple calls
4. **Testing**: Test with different input values to ensure your functions work correctly
5. **Documentation**: Document any special imports or type requirements

## Example Output

When working correctly, this project produces:

```
Add: 10 + 5 = 15
Subtract: 10 - 5 = 5
Multiply: 10 * 5 = 50
Divide: 10 / 3 = 3
Modulo: 10 % 3 = 1
```

## Useful Tools

- **wasm-tools**: For inspecting WASM modules (`cargo install wasm-tools`)
- **wasmtime**: Runtime for executing WASM
- **wasm-bindgen**: For generating JavaScript bindings (if targeting web)

## Further Reading

- [WebAssembly Specification](https://webassembly.github.io/spec/)
- [Wasmtime Documentation](https://docs.wasmtime.dev/)
- [wasm-bindgen Book](https://rustwasm.github.io/wasm-bindgen/)
- [Rust and WebAssembly Book](https://rustwasm.github.io/docs/book/) 