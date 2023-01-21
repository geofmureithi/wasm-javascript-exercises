# Exercise 1
The theme here is to get ourselves introduced to WASM.

Time: 10 min

## Part 1
Let's start with working on the js part. Our goal here is to implement the node.js approach for a fibonacci sequence.

Expected outcome:
- The fibonacci method in index.js can calculate fibonacci numbers successfully

Marks: 4


## Part 2
Here we look at the part of the rust's implementation.

Expected outcome:
- Running `cargo test` in the rust folder passes after verifying the logic

Marks: 6


## Part 3
Export the rust implementation into a WASM file using `wasm-pack build` in the rust folder.
Write a simple script that calculates the time to calculates the value of `fibonacci(34)` for both the wasm and js implementation.

Marks: 4

## Further Reading