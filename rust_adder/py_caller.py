from ctypes import cdll, c_int

# Load the library (use the appropriate file name for your OS)
lib = cdll.LoadLibrary('./target/release/librust_adder.dylib')  # or .dylib, .dll

# Set the argument and return types
lib.add_numbers.argtypes = (c_int, c_int)
lib.add_numbers.restype = c_int

# Call the function
result = lib.add_numbers(5, 7)
print(f"The result is: {result}")
