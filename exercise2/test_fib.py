# Import the custom loader for `*.wasm` files
import wasmtime.loader

# Assuming `rust_fibonacci.wasm` is in the python load path...
import rust_fibonacci

# Fibonacci implementation from wasm file 
# See also https://mureithi.me/blog/the-perfect-tribonacci-sequence
def fibonacci(n):
    return rust_fibonacci.fibonacci(n)

def test_fib():
    assert(fibonacci(5) == 5)
    assert(fibonacci(9) == 34)