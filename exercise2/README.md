# Exercise 2

Time: 7 min

Here we are going to look at how we can use our exported wasm file on other platforms.
We can use it in many languages such as nodejs, go, python, php and so much more.

This task is a `free flow one` where you can choose the language of your choice.

The current example in python shows how to go about it:
- Get the `wasmtime` loader for your laguage
- Import the wasm file we build in Exercise 1
- Load the fibonacci method and call it

Expected Outcome: The .wasm file will be imported and called, and there will be assertions to verify the sanity.

4 marks

## Solution
Run:
```
cp ./exercise1/rust/pkg/rust_fibonacci_bg.wasm ./exercise2/rust_fibonacci.wasm
python3 -m pytest test_fib.py
```

## Further reading
[Awesome WebAssembly Runtimes](https://github.com/appcypher/awesome-wasm-runtimes)
[Wasmtime](https://github.com/bytecodealliance/wasmtime)