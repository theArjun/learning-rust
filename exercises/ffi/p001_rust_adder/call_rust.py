from ctypes import cdll, c_int
from pathlib import Path

DYLIB_PATH = Path.cwd() / "target" / "release" / "libp001_rust_adder.dylib"

# Load the shared library. Adjust the path as necessary.
lib = cdll.LoadLibrary(str(DYLIB_PATH))

add_numbers = lib.add_numbers

# Set the argument and return types
add_numbers.argtypes = (c_int, c_int)
add_numbers.restype = c_int

# Call the Rust function
result = lib.add_numbers(5, 7)
print(f"The result is: {result}")
