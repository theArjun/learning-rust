# Rust FFI Adder Library

A simple Foreign Function Interface (FFI) example demonstrating how to call Rust functions from other languages.

## Overview

This project creates a C-compatible dynamic library from Rust code that exports basic arithmetic functions. The library can be called from any language that supports C FFI, such as Python, C, C++, etc.

## Functions Exported

- `add_numbers(a: i32, b: i32) -> i32` - Adds two 32-bit integers
- `subtract_numbers(a: i32, b: i32) -> i32` - Subtracts two 32-bit integers

## Building the Library

### Prerequisites
- Rust toolchain installed
- Python 3.x (for the example)

### Build Steps

1. **Build the dynamic library:**
   ```bash
   cargo build --release
   ```

   This creates a dynamic library at:
   - **Linux:** `./target/release/libp001_rust_adder.so`
   - **macOS:** `./target/release/libp001_rust_adder.dylib`
   - **Windows:** `./target/release/p001_rust_adder.dll`

## Usage Examples

### Python

The included `call_rust.py` demonstrates basic usage:

```python
from ctypes import cdll, c_int

# Load the shared library
lib = cdll.LoadLibrary("./target/release/libp001_rust_adder.dylib")  # macOS
# lib = cdll.LoadLibrary("./target/release/libp001_rust_adder.so")     # Linux
# lib = cdll.LoadLibrary("./target/release/p001_rust_adder.dll")       # Windows

# Configure function signatures
lib.add_numbers.argtypes = (c_int, c_int)
lib.add_numbers.restype = c_int

lib.subtract_numbers.argtypes = (c_int, c_int)
lib.subtract_numbers.restype = c_int

# Call the functions
result_add = lib.add_numbers(5, 7)
result_sub = lib.subtract_numbers(10, 3)

print(f"5 + 7 = {result_add}")
print(f"10 - 3 = {result_sub}")
```

### Running the Python Example

```bash
# Build the library first
cargo build --release

# Run the Python example
python call_rust.py
```

## Key FFI Concepts Demonstrated

- **`#[no_mangle]`**: Prevents Rust from mangling the function names, making them accessible from C
- **`extern "C"`**: Uses C calling convention for compatibility
- **`cdylib`**: Cargo configuration to build a C-compatible dynamic library
- **ctypes**: Python's foreign function library for calling C functions

## Platform Considerations

- Library file extensions vary by platform (.so, .dylib, .dll)
- Update the library path in your calling code based on your target platform
- The example Python script assumes macOS (.dylib extension)

## Troubleshooting

- Ensure the library path in your calling code matches the actual generated library location
- Check that the target architecture matches between the Rust library and calling program
- Verify function signatures match between Rust and calling language declarations
