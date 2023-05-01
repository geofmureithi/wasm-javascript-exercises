# Import the custom loader for `*.wasm` files
import wasmtime.loader

# Assuming `rust_fibonacci.wasm` is in the python load path...
# ?? import file

# Fibonacci implementation from wasm file 
# See also https://mureithi.me/blog/the-perfect-tribonacci-sequence
def fibonacci(n):
    a, b = 0, 1
    if n == 0:
        return a
    for i in range(2, n + 1):
        c = a + b
        a = b
        b = c
    return b

def test_fib():
    assert(fibonacci(5) == 5)
    assert(fibonacci(9) == 34)