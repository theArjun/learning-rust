from ctypes import cdll, c_int
import time


# Load the library (use the appropriate file name for your OS)
lib = cdll.LoadLibrary('./target/release/librust_adder.dylib')  # or .dylib, .dll

# Set the argument and return types
lib.add_numbers.argtypes = (c_int, c_int)
lib.add_numbers.restype = c_int

# Call the function
start_time = time.perf_counter()
result = lib.add_numbers(5, 7)
end_time = time.perf_counter()

rust_time = end_time - start_time

print("From rust: ", end_time - start_time)


def add_numbers(a, b):
    return a + b


# In python
start_time = time.perf_counter()
for i in range(100_000_000):
    result = add_numbers(5, 7)
end_time = time.perf_counter()
python_time = end_time - start_time

rust_is_fast_by = python_time / rust_time * 100
print(f"Rust is fast by : {rust_is_fast_by:2f}")



